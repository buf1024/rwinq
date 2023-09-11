import json
from typing import Dict, Optional
from pywqdata.loader import Loader
from pywqfetch.fetch import Fetch
from pywqstrategy.strategy import CommonParam, Strategy, StrategyResult


class TestStrategy(Strategy):
    def __init__(self, loader: Optional[Loader] = None, fetch: Optional[Fetch] = None, cmm_params: Optional[CommonParam] = None, params: Optional[Dict] = None) -> None:
        super().__init__(loader=loader, fetch=fetch, cmm_params=cmm_params, params=params)

    def help(self):
        return "Python 测试策略"

    def name(self):
        return "TestStrategy"

    async def prepare(self) -> bool:
        print("python prepare, cmm_params: {}, params: {}".format(
            self.cmm_params, self.params))
        return True

    async def test(self, typ, code, name) -> Optional[str]:
        # print("test in python, typ={}, code={}, name={}".format(typ, code, name))
        codes = [
            "sz002805",
            "sz300827",
            "sz000762",
        ]
        if code in codes:
            print('python got data: {}'.format(code))
            stat = None
            mark = None
            if self.loader is not None:
                data = await self.loader.load_stock_daily(filter={'code': code}, sort={'trade_date': -1}, limit=15, to_frame=False)

                stat = self.stat_result(data, 10, 10)
                mark = {}
                mark[data[0]['trade_date'].date()] = 'mark1'
                mark[data[2]['trade_date'].date()] = 'mark2'
            return StrategyResult(code=code, name=name, mark=mark, stat=stat)

        return None
