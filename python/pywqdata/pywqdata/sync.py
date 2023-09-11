from typing import List, Optional, Tuple
from pywqdata.pywqdata import Sync, BlockSync


class Dest:
    def __init__(self,
                 *,
                 file: Optional[str] = None,
                 mongo: Optional[str] = None,
                 mysql: Optional[str] = None,
                 clickhouse: Optional[str] = None) -> None:
        self.file = file
        self.mongo = mongo
        self.mysql = mysql
        self.clickhouse = clickhouse

    def to_list(self) -> Optional[List[Tuple]]:
        list = []
        if self.file is not None:
            list.append(('file', self.file))
        if self.mongo is not None:
            list.append(('mongodb', self.mongo))
        if self.mysql is not None:
            list.append(('mysql', self.mysql))
        if self.clickhouse is not None:
            list.append(('clickhouse', self.clickhouse))
        return list


class Funcs:
    TradeDate = 1
    # index
    IndexInfo = 2
    IndexDaily = 3
    # stock
    StockInfo = 4
    StockBar = 5
    StockIndex = 6
    StockIndustry = 7
    StockIndustryDetail = 8
    StockIndustryBar = 9,
    StockConcept = 10,
    StockConceptDetail = 11
    StockConceptBar = 12,
    StockYJBB = 13
    StockMargin = 14

    # fund
    FundInfo = 15
    FundNet = 16
    FundBar = 17

    # bond
    BondInfo = 18
    BondBar = 19


class MySync:
    def __init__(self, dest: Dest, funcs: Optional[List[int]] = None):
        self.inner = Sync(dest.to_list(), funcs)

    async def sync(self, skip_basic=False, task_count=4, split_count=5):
        await self.inner.sync(skip_basic, task_count, split_count)

    def shutdown(self):
        self.inner.shutdown()


class MyBlockSync:
    def __init__(self, dest: Dest, funcs: Optional[List[int]] = None):
        self.inner = BlockSync(dest.to_list(), funcs)

    def sync(self, skip_basic=False, task_count=4, split_count=5):
        self.inner.sync(skip_basic, task_count, split_count)

    def shutdown(self):
        self.inner.shutdown()
