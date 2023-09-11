import json
from typing import Dict, List, Optional

from rwqpy.strategy.base_strategy import BaseStrategy
from pywqdata.rwqloader import Loader
from rwqpyfetch.rwqfetch import Fetch
from pywqstrategy.strategy import CommonParam, StrategyResult, StrategyType


class ShockRise(BaseStrategy):
    """
    左侧震荡后，开始突破
    示意形态:
         |
        |
    ||||
    """

    def __init__(self, strategy_type: List[int] = [StrategyType.Stock], loader: Optional[Loader] = None, fetch: Optional[Fetch] = None, cmm_params: Optional[CommonParam] = None, params: Optional[Dict] = None) -> None:
        super().__init__(strategy_type, loader, fetch, cmm_params, params)
        self.min_break_days = 3
        self.min_break_up_pct = 5.0
        self.min_shock_days = 15
        self.max_shock_per_day_pct = 7.0
        self.max_shock_pct = 15.0

    @staticmethod
    def help():
        return '  名称: 底部横盘突破选股(基于日线)\n' + \
               '  说明: 选择底部横盘的股票\n' + \
               '  参数: min_break_days -- 最近突破上涨天数(默认: 3)\n' + \
               '        min_break_up_pct -- 最近累计突破上涨百分比(默认: 5.0)\n' + \
               '        min_shock_days -- 最小横盘天数(默认: 15)\n' + \
               '        max_shock_per_day_pct -- 横盘天数内每天波动百分比(默认: 7.0)\n' + \
               '        max_shock_pct -- 涨盘天数内总波动百分比(默认: 15.0)'

    async def prepare(self) -> bool:
        if self.params is not None:
            try:
                if 'min_break_days' in self.params:
                    self.min_break_days = int(self.params['min_break_days'])
                if 'min_break_up_pct' in self.params:
                    self.min_break_up_pct = float(
                        self.params['min_break_up_pct'])
                if 'max_upper_shadow_pct' in self.params:
                    self.max_upper_shadow_pct = float(
                        self.params['max_upper_shadow_pct'])
                if 'min_shock_days' in self.params:
                    self.min_shock_days = int(self.params['min_shock_days'])
                    if self.min_shock_days > self.cmm_params.test_trade_days:
                        self.logger.error('min_shock_days{} 应比cmm_params.test_trade_days{}小'.format(
                            self.min_shock_days, self.cmm_params.test_trade_days))
                        return False
                if 'max_shock_per_day_pct' in self.params:
                    self.max_shock_per_day_pct = float(
                        self.params['max_shock_per_day_pct'])
                if 'max_shock_pct' in self.params:
                    self.max_shock_pct = float(self.params['max_shock_pct'])

            except ValueError:
                self.logger.error('策略参数不合法')
                return False
        return True

    async def test(self, typ: StrategyType, code: str, name: str) -> Optional[StrategyResult]:
        self.logger.debug(
            'testing typ: {}, code: {}, name: {}'.format(typ, code, name))

        if self.min_shock_days > self.cmm_params.test_trade_days:
            return None

        kdata = await self.load_kdata(typ=typ,
                                      filter={'code': code,
                                              'trade_date': {'$lte': self.cmm_params.test_end_date}},
                                      limit=self.cmm_params.test_trade_days,
                                      sort={'trade_date': -1},
                                      to_frame=False)

        if kdata is None or len(kdata) < self.cmm_params.test_trade_days:
            return None

        n_close = kdata[0]['close']

        hit_days = 0
        hit, hit_max = 0, 0
        break_up_pct = 0.0

        for (index, data) in enumerate(kdata):
            chg_pct, close, open, low, high = data['chg_pct'], data[
                'close'], data['open'], data['low'], data['high']

            if index + 1 < len(kdata):
                last_high = kdata[index + 1]['high']
                if chg_pct >= 0 and \
                        close > open and \
                        last_high > low:

                    hit_days = hit_days + 1
                    break_up_pct = break_up_pct + chg_pct
                    if hit_days == self.min_break_days:
                        hit = index
                    hit_max = index
                    continue
            break

        if hit_days < self.min_break_days:
            return None

        if break_up_pct < self.min_break_up_pct:
            return None

        test_data = kdata[hit_max + 1:]
        if len(test_data) < self.min_shock_days:
            return None

        shock_hit_days = 0
        shock_pct = 0.0
        shock_index, shock_max_index = 0, 0
        for (index, data) in enumerate(test_data):
            chg_pct = abs(data['chg_pct'])
            high = data['high']
            if chg_pct < self.max_shock_per_day_pct and \
                    high < n_close:
                shock_hit_days = shock_hit_days + 1
                if index < self.min_shock_days:
                    shock_index = index + hit_max + 1
                    shock_pct = shock_pct + data['chg_pct']
                shock_max_index = index + hit_max + 1
                continue
            break

        shock_pct = abs(shock_pct)
        if shock_pct > self.max_shock_pct or shock_hit_days < self.min_shock_days:
            return None

        stat = self.stat_result(kdata, hit, hit_max)
        mark = {}
        mark[kdata[hit]['trade_date'].date()] = 'hit'
        mark[kdata[hit_max]['trade_date'].date()] = 'hit_max'
        mark[kdata[shock_index]['trade_date'].date()] = 'shock_start'
        mark[kdata[shock_max_index]['trade_date'].date()] = 'shock_end'

        return StrategyResult(code=code, name=name, mark=mark, stat=stat)


if __name__ == '__main__':
    import logging
    import nest_asyncio

    from rwqpy.common import run_until_complete
    from pywqdata import get_loader
    from pywqstrategy import Runner

    nest_asyncio.apply()

    FORMAT = '[%(asctime)-15s][%(filename)s:%(lineno)d][%(name)s][%(levelname)s] %(message)s'
    logging.basicConfig(encoding='utf-8', format=FORMAT, level=logging.INFO)

    async def test_strategy():
        loader = get_loader('mongodb', 'mongodb://localhost:27017')
        strategy = ShockRise(loader=loader)
        runner = Runner('mongodb', 'mongodb://localhost:27017', 50)

        data = await runner.run(strategy)

        if data is not None:
            data = data[StrategyType.Stock]
            for (index, res) in enumerate(data):
                print('{}#{}'.format(index, res.to_dict()))
        else:
            print('no data')

    run_until_complete(test_strategy())
