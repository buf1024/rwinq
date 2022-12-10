from ast import Dict
from typing import Optional
from hiq_pystrategy.strategy import Strategy, StrategyType

def stat_result(data: str, hit: int, hit_max: int) -> Dict:
    pass

class Runner:
    def __init__(self, typ: str, url: str, concurrent: int):
        pass

    async def run(self, strategy: Strategy, codes: Optional[Dict]) -> Optional[Dict]:
        pass
    
    async def fit(self, strategy: Strategy, code: str, name: str, typ: StrategyType) -> Optional[Dict]:
        pass

    def shutdown(self) -> bool:
        pass
