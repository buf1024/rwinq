import pandas as pd
from types import FunctionType
from typing import Dict, Optional
from pywqstrategy.strategy import Strategy, StrategyResult, StrategyType
from pywqstrategy.pywqstrategy import Runner


def _progress_callback(code: str, name: str, total: int, current: int, progress: float):
    print('\r>> processing {}({}) {}/{} {}%          '.format(name,
          code, current, total, round(progress, 2)), end='')
    if total == current:
        print('')


class Runner:
    def __init__(self, typ: str, url: str, concurrent: int = 50):
        self.inner = Runner(typ=typ, url=url, concurrent=concurrent)

    @property
    def runner(self):
        return self.inner

    @staticmethod
    def progress_callback(code: str, name: str, total: int, current: int, progress: float):
        return _progress_callback(code=code, name=name, total=total, current=current, progress=progress)

    async def run(self, strategy: Strategy, codes: Optional[Dict] = None, progress_func: Optional[FunctionType] = _progress_callback, to_frame: bool = False) -> Optional[Dict]:
        data = await self.inner.run(strategy, codes, progress_func)
        if data is not None:
            m = {}
            for (k, v) in data.items():
                l = []
                for e in v:
                    rst = StrategyResult()
                    rst.from_json(e)
                    l.append(rst.to_plain_dict()
                             ) if to_frame else l.append(rst)
                m[k] = pd.DataFrame(l) if to_frame else l
            data = m
        return data

    async def fit(self, strategy: Strategy, code: str, name: str, typ: StrategyType, to_frame: bool = False) -> Optional[StrategyResult]:
        data = await self.inner.fit(strategy, code, name, typ)
        if data is not None:
            rst = StrategyResult()
            rst.from_json(data)
            data = pd.DataFrame([rst.to_plain_dict()]) if to_frame else rst
        return data

    def shutdown(self) -> bool:
        return self.inner.shutdown()
