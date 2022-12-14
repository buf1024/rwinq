import json
from typing import Dict, List, Optional

from hiq_py.strategy.base_strategy import BaseStrategy
from hiq_pydata.hiq_loader import HiqLoader
from hiq_pyfetch.hiq_fetch import HiqFetch
from hiq_pystrategy.strategy import CommonParam, StrategyResult, StrategyType


class RightSide(BaseStrategy):
    """
    看上方有没有抛压，上涨幅度。
    貌似上涨30%的都可以追，上涨大于40%有一定下跌的概率
    示意形态:
       |
      |
     |
    |
    """

    def __init__(self, strategy_type: List[int] = [StrategyType.Stock], loader: Optional[HiqLoader] = None, fetch: Optional[HiqFetch] = None, cmm_params: Optional[CommonParam] = None, params: Optional[Dict] = None) -> None:
        super().__init__(strategy_type, loader, fetch, cmm_params, params)

        self.min_rise_days = 3
        self.max_shadow_pct = 50.0
        self.min_test_days = 30

    @staticmethod
    def help() -> str:
        return '  名称: 右侧策略(基于日线)\n' + \
               '  说明: 选择右侧上涨的标的\n' + \
               '  参数: min_rise_days -- 最近最小连续上涨天数(默认: 3)\n' + \
               '        max_shadow_pct -- 上下影线最大百分比(默认: 50.0)\n' + \
               '        min_test_days -- 最小成交量增加百分比(默认: 30)\n'

    async def prepare(self) -> bool:
        if self.params is not None:
            try:
                if 'min_rise_days' in self.params:
                    self.min_rise_days = int(self.params['min_rise_days'])
                if 'max_shadow_pct' in self.params:
                    self.max_shadow_pct = float(self.params['max_shadow_pct'])
                if 'min_test_days' in self.params:
                    self.min_test_days = int(
                        self.params['min_test_days'])
            except ValueError:
                self.logger.error('策略参数不合法')
                return False
        return True

    async def test(self, typ: StrategyType, code: str, name: str) -> Optional[StrategyResult]:
        self.logger.debug(
            'testing typ: {}, code: {}, name: {}'.format(typ, code, name))
        kdata = await self.load_kdata(typ=typ,
                                      filter={'code': code,
                                              'trade_date': {'$lte': self.cmm_params.test_end_date}},
                                      limit=self.cmm_params.test_trade_days,
                                      sort={'trade_date': -1},
                                      to_frame=False)

        if kdata is None or len(kdata) < self.cmm_params.test_trade_days:
            return None

        hit_days = 0
        hit, hit_max = 0, 0
        for (index, data) in enumerate(kdata):
            chg_pct = data['chg_pct']
            open, close, high, low = data['open'], data['close'], data['high'], data['low']

            last_close = close/(1+chg_pct/100.0)
            _, u_shadow, _, _ = self.shadow(last_close,
                                            open, close, low, high)
            if chg_pct > 0 and \
                    low < last_close and \
                    u_shadow <= self.max_shadow_pct:
                hit_days = hit_days + 1
                if hit_days == self.min_rise_days:
                    hit = index
                hit_max = index
                continue
            break

        if hit_days < self.min_rise_days:
            return None

        test_data = kdata[self.min_rise_days:]
        t_close = kdata[hit]['close']
        t_days = 0
        t_days_index = 0
        for (index, data) in enumerate(test_data):
            close = data['close']
            if t_close > close:
                t_days_index = index
                t_days = t_days + 1
                continue
            break
        if t_days < self.min_test_days:
            return None
        t_days_index = t_days_index + self.min_rise_days
        
        stat = self.stat_result(kdata, hit, hit_max)
        mark = {}
        mark[kdata[hit]['trade_date'].date()] = json.dumps(
            kdata[hit], default=self.json_def_handler)
        mark[kdata[hit_max]['trade_date'].date()] = json.dumps(
            kdata[hit_max], default=self.json_def_handler)
        mark[kdata[t_days_index]['trade_date'].date()] = json.dumps(
            kdata[t_days_index], default=self.json_def_handler)

        return StrategyResult(code=code, name=name, mark=mark, stat=stat)


if __name__ == '__main__':
    import logging
    import nest_asyncio

    from hiq_py.common import run_until_complete
    from hiq_pydata import get_loader
    from hiq_pystrategy import HiqRunner

    nest_asyncio.apply()

    FORMAT = '[%(asctime)-15s][%(filename)s:%(lineno)d][%(name)s][%(levelname)s] %(message)s'
    logging.basicConfig(encoding='utf-8', format=FORMAT, level=logging.INFO)

    async def test_strategy():
        loader = get_loader('mongodb', 'mongodb://localhost:27017')
        strategy = RightSide(loader=loader)
        runner = HiqRunner('mongodb', 'mongodb://localhost:27017', 50)

        data = await runner.run(strategy)

        if data is not None:
            data = data[StrategyType.Stock]
            for (index, res) in enumerate(data):
                print('{}#{}'.format(index, res.to_dict()))
        else:
            print('no data')

    run_until_complete(test_strategy())
