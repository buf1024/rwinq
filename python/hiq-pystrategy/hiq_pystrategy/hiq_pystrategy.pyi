from ast import Dict
from typing import Optional
from hiq_pystrategy.strategy import Strategy

def stat_result(data: str, hit: int, hit_max: int) -> Dict:
    pass

class Runner:
    def __init__(self, typ: str, url: str, concurrent: int):
        pass

    async def run(self, strategy: Strategy, codes: Optional[Dict]) -> Optional[Dict]:
        pass

    def shutdown(self) -> bool:
        pass
