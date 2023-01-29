import json
from typing import Dict, List, Optional

from hiq_py.strategy.base_strategy import BaseStrategy
from hiq_pydata.hiq_loader import HiqLoader
from hiq_pyfetch.hiq_fetch import HiqFetch
from hiq_pystrategy.strategy import CommonParam, StrategyResult, StrategyType
from hiq_pystrategy import ta_ma


class MaTurn(BaseStrategy):
    """
    根据ma拐
    """

    def __init__(self, strategy_type: List[int] = [StrategyType.Stock], loader: Optional[HiqLoader] = None, fetch: Optional[HiqFetch] = None, cmm_params: Optional[CommonParam] = None, params: Optional[Dict] = None) -> None:
        super().__init__(strategy_type, loader, fetch, cmm_params, params)

        self.ma = 5
        self.min_up_days = 10
        self.min_up_pct = 20.0
        self.min_down_days = 5
        self.min_turn_days = 2
        self.max_turn_days = 5

    @staticmethod
    def help() -> str:
        return '  名称: 日线左右拐(基于日线)\n' + \
               '  说明: 选择右侧上涨的标的\n' + \
               '  参数: ma类型 -- ma类型，如： 5， 10， 15， 20等(默认: 5)\n' + \
               '        min_up_days -- 高点前上涨的天数(默认: 10)\n' + \
               '        min_up_pct -- 高点前上涨福(默认: 20)\n' + \
               '        min_down_days -- 上拐测试期间涨跌天数(默认: 5)\n' + \
               '        min_turn_days -- 出现上拐最小天数(默认: 2)\n' + \
               '        max_turn_days -- 出现上拐最大天数(默认: 5)\n'

    async def prepare(self) -> bool:
        if self.params is not None:
            try:
                if 'ma' in self.params:
                    self.ma = int(self.params['ma'])
                if 'min_up_days' in self.params:
                    self.min_up_days = int(self.params['min_up_days'])
                if 'min_up_pct' in self.params:
                    self.min_up_pct = float(self.params['min_up_pct'])
                if 'min_down_days' in self.params:
                    self.min_down_days = int(
                        self.params['min_down_days'])
                if 'min_turn_days' in self.params:
                    self.min_turn_days = int(
                        self.params['min_turn_days'])
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

        close = [data['close'] for data in kdata]
        ma = ta_ma(close, self.ma)

        n = len(ma) - self.ma - 1
        hit_days = 0
        hit = 0
        for curr in range(0, n):
            prev = curr + 1
            if ma[curr] > ma[prev]:
                hit_days += 1
                hit += 1
                continue
            break

        if hit_days < self.min_turn_days or hit_days > self.max_turn_days:
            return None

        hit2 = hit
        hit_days = 0
        for curr in range(hit + 1, n):
            prev = curr + 1
            if ma[curr] < ma[prev]:
                hit_days += 1
                hit2 += 1
                continue
            break

        if hit_days < self.min_down_days:
            return None

        hit3 = hit2
        hit_days = 0
        for curr in range(hit2 + 1, n):
            prev = curr + 1
            if ma[curr] > ma[prev]:
                hit_days += 1
                hit3 += 1
                continue
            break

        if hit_days < self.min_up_days:
            return None

        mark = {}
        mark[kdata[hit]['trade_date'].date()] = json.dumps(
            kdata[hit], default=self.json_def_handler)
        mark[kdata[hit2]['trade_date'].date()] = json.dumps(
            kdata[hit2], default=self.json_def_handler)
        mark[kdata[hit3]['trade_date'].date()] = json.dumps(
            kdata[hit3], default=self.json_def_handler)

        return StrategyResult(code=code, name=name, mark=mark, stat=None)


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
        strategy = MaTurn(loader=loader)
        runner = HiqRunner('mongodb', 'mongodb://localhost:27017', 50)

        data = await runner.run(strategy)

        if data is not None:
            data = data[StrategyType.Stock]
            for (index, res) in enumerate(data):
                print('{}#{}'.format(index, res.to_dict()))
        else:
            print('no data')

    run_until_complete(test_strategy())
