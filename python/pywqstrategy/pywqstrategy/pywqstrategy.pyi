from ast import Dict
from types import FunctionType
from typing import List, Optional
from pywqstrategy.strategy import Strategy, StrategyType


def ta_ma(bar: List[float], ma_type: int) -> List[float]:
    pass


def stat_result(data: str, hit: int, hit_max: int) -> Dict:
    pass


class Runner:
    def __init__(self, typ: str, url: str, concurrent: int):
        pass

    async def run(self, strategy: Strategy, codes: Optional[Dict] = None, progress_func: Optional[FunctionType] = None) -> Optional[Dict]:
        pass

    async def fit(self, strategy: Strategy, code: str, name: str, typ: StrategyType) -> Optional[Dict]:
        pass

    def shutdown(self) -> bool:
        pass
