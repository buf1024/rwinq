from abc import ABC
from datetime import date
import json
from typing import Dict, List, Optional
from hiq_pydata.hiq_loader import HiqLoader
from hiq_pyfetch.hiq_fetch import HiqFetch
from hiq_pystrategy.hiq_pystrategy import stat_result

def _json_def_handler(obj):
    if hasattr(obj, 'isoformat'):
        return obj.isoformat()
    return None

class StrategyType:
    Bond = 1
    Fund = 2
    Stock = 3
    Index = 4
    Concept = 5
    Industry = 6

    @staticmethod
    def str(typ) -> str:
        m = {
            StrategyType.Bond: 'Bond',
            StrategyType.Fund: 'Fund',
            StrategyType.Stock: 'Stock',
            StrategyType.Index: 'Index',
            StrategyType.Concept: 'Concept',
            StrategyType.Industry: 'Industry'
        }
        if typ in m.keys():
            return m[typ]
        return ''


class Stat:
    def __init__(self, hit_chg_pct=None, start=None, end=None, low=None, high=None, hit=None, hit_max=None) -> None:
        self.hit_chg_pct = hit_chg_pct
        self.start = start
        self.end = end
        self.low = low
        self.high = high
        self.hit = hit
        self.hit_max = hit_max

    def to_dict(self):
        return dict(hit_chg_pct=self.hit_chg_pct,
                    start=self.start,
                    end=self.end,
                    low=self.low,
                    high=self.high,
                    hit=self.hit,
                    hit_max=self.hit_max)

    def from_json(self, js):
        self.hit_chg_pct = js['hit_chg_pct']
        self.start = js['start']
        self.end = js['end']
        self.low = js['low']
        self.high = js['high']
        self.hit = js['hit']
        self.hit_max = js['hit_max']

    def to_json(self):
        return json.dumps(self.to_dict(), default=_json_def_handler)


class StrategyResult:
    def __init__(self, code=None, name=None, mark=None, stat=None) -> None:
        self.code = code
        self.name = name
        self.mark = mark
        self.stat = stat

    def to_dict(self):
        return dict(code=self.code, name=self.name, stat=self.stat.to_dict() if self.stat is not None else None)

    def from_json(self, js):
        self.code = js['code']
        self.name = js['name']
        if 'mark' in js:
            self.mark = js['mark']

        if 'stat' in js:
            s = Stat()
            s.from_json(js['stat'])
            self.stat = s

    def to_json(self):
        return json.dumps(self.to_dict(), default=_json_def_handler)


class CommonParam:
    def __init__(self, test_end_date, test_trade_days) -> None:
        self.test_end_date = test_end_date
        self.test_trade_days = test_trade_days


class Strategy(ABC):
    def __init__(self, loader: Optional[HiqLoader] = None, fetch: Optional[HiqFetch] = None, cmm_params: Optional[CommonParam] = None, params: Optional[Dict] = None) -> None:
        self.loader = loader
        self.fetch = fetch
        self.cmm_params = cmm_params
        self.params = params

    async def run(self, typ, code, name) -> Optional[str]:
        """
        由runner调用，不要重写
        """
        rs = await self.test(typ, code, name)
        if rs is not None:
            rs = rs.to_json()
        return rs

    @staticmethod
    def stat_result(data: List[Dict], hit: int, hit_max: int) -> Stat:
        js_str = json.dumps(data, default=_json_def_handler)
        js_str = stat_result(js_str, hit, hit_max)
        s = Stat()
        s.from_json(js_str)
        return s

    def help(self) -> str:
        return ""

    def name(self) -> str:
        return self.__qualname__.name

    def accept(self) -> List[int]:
        return [StrategyType.Stock]

    async def prepare(self) -> bool:
        return True

    async def test(self, typ: StrategyType, code: str, name: str) -> Optional[StrategyResult]:
        pass
