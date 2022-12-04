from abc import ABC
import json
from typing import List, Optional


class StrategyType:
    Bond = 1
    Fund = 2
    Stock = 3
    Index = 4
    Concept = 5
    Industry = 6


class Marker:
    def __init__(self, key=None, val=None) -> None:
        self.key = key
        self.val = val

    def to_dict(self):
        return dict(key=self.key, val=self.val)

    def from_json(self, js):
        self.key = js['key']
        self.val = js['val']

    def to_json(self):
        return json.dumps(self.to_dict())


class Stat:
    def __init__(self, chg_pct=None) -> None:
        self.chg_pct

    def to_dict(self):
        return dict(chg_pct=self.chg_pct)

    def from_json(self, js):
        self.chg_pct = js['chg_pct']

    def to_json(self):
        return json.dumps(self.to_dict())


class StrategyResult:
    def __init__(self, code=None, name=None, marker=None, stat=None) -> None:
        self.code = code
        self.name = name
        self.marker = marker
        self.stat = stat

    def to_dict(self):
        return dict(code=self.code, name=self.name, marker=self.marker.to_dict() if self.marker is not None else None, stat=self.stat.to_dict() if self.stat is not None else None)

    def from_json(self, js):
        self.code = js['code']
        self.name = js['name']
        if 'marker' in js:
            m = Marker()
            m.from_json(js['marker'])
            self.marker = m
        if 'stat' in js:
            s = Stat()
            s.from_json(js['stat'])
            self.stat = s

    def to_json(self):
        return json.dumps(self.to_dict())


class Strategy(ABC):
    def __init__(self, loader, fetch, cmm_params, params) -> None:
        self.loader = loader
        self.fetch = fetch
        self.cmm_params = cmm_params
        self.params = params

    def help(self) -> str:
        return ""

    def name(self) -> str:
        return ""

    def accept(self) -> List[int]:
        return [StrategyType.Stock]

    async def run(self, typ, code, name) -> Optional[str]:
        rs = await self.test(typ, code, name)
        if rs is not None:
            rs = rs.to_json()
        return rs

    async def prepare(self) -> bool:
        return True

    async def test(self, typ, code, name) -> Optional[StrategyResult]:
        pass
