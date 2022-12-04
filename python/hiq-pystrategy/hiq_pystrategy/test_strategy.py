from typing import Optional
from hiq_pystrategy.strategy import Strategy, StrategyResult


class TestStrategy(Strategy):
    def __init__(self, loader, fetch, cmm_params, params) -> None:
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
            return StrategyResult(code=code, name=name, marker=None, stat=None)

        return None
