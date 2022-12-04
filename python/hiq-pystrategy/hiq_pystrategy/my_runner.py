
from typing import Dict, Optional
from hiq_pystrategy.strategy import Strategy, StrategyResult
from hiq_pystrategy.hiq_pystrategy import Runner


class MyRunner:
    def __init__(self, typ: str, url: str, concurrent: int):
        self.inner = Runner(typ=typ, url=url, concurrent=concurrent)

    @property
    def runner(self):
        return self.inner

    async def run(self, strategy: Strategy) -> Optional[Dict]:
        data = await self.inner.run(strategy)
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

    def shutdown(self) -> bool:
        pass
