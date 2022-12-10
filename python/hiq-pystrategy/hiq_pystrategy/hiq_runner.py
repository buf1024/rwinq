
from typing import Dict, Optional
from hiq_pystrategy.strategy import Strategy, StrategyResult, StrategyType
from hiq_pystrategy.hiq_pystrategy import Runner


class HiqRunner:
    def __init__(self, typ: str, url: str, concurrent: int):
        self.inner = Runner(typ=typ, url=url, concurrent=concurrent)

    @property
    def runner(self):
        return self.inner

    async def run(self, strategy: Strategy, codes: Optional[Dict] = None) -> Optional[Dict]:
        data = await self.inner.run(strategy, codes)
        if data is not None:
            m = {}
            for (k, v) in data.items():
                l = []
                for e in v:
                    rst = StrategyResult()
                    rst.from_json(e)
                    l.append(rst)
                m[k] = l
            data = m
        return data

    async def fit(self, strategy: Strategy, code: str, name: str, typ: StrategyType) -> Optional[StrategyResult]:
        data = await self.inner.fit(strategy, code, name, typ)
        if data is not None:
            rst = StrategyResult()
            rst.from_json(data)
            data = rst
        return data

    def shutdown(self) -> bool:
        return self.inner.shutdown()
