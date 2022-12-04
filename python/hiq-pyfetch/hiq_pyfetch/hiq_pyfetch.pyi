from typing import List, Dict, Optional, Set
from datetime import date


async def fetch_trade_date() -> Set[int]:
    pass

def block_fetch_trade_date() -> Set[int]:
    pass


def to_std_code(int, str) -> str:
    pass


class BondFetch:
    def __init__(self):
        pass

    async def fetch_bond_info(self) -> List[Dict]:
        pass

    async def fetch_bond_bar(self, code: str, name: str,
                       stock_code: str, stock_name: str,
                       freq: Optional[int],
                       start: Optional[date], end: Optional[date]) -> Dict:
        pass
    
class BlockBondFetch:
    def __init__(self):
        pass

    async def fetch_bond_info(self) -> List[Dict]:
        pass

    async def fetch_bond_bar(self, code: str, name: str,
                       stock_code: str, stock_name: str,
                       freq: Optional[int],
                       start: Optional[date], end: Optional[date]) -> Dict:
        pass


class FundFetch:
    def __init__(self):
        pass

    async def fetch_fund_info(self) -> List[Dict]:
        pass

    async def fetch_fund_net(self, code: str, name: Optional[str],
                       start: Optional[date], end: Optional[date]) -> List[Dict]:
        pass

    async def fetch_fund_bar(self, code: str, name: Optional[str],
                       freq: Optional[int],
                       start: Optional[date], end: Optional[date]) -> Dict:
        pass

class BlockFundFetch:
    def __init__(self):
        pass

    def fetch_fund_info(self) -> List[Dict]:
        pass

    def fetch_fund_net(self, code: str, name: Optional[str],
                       start: Optional[date], end: Optional[date]) -> List[Dict]:
        pass

    def fetch_fund_bar(self, code: str, name: Optional[str],
                       freq: Optional[int],
                       start: Optional[date], end: Optional[date]) -> Dict:
        pass




class StockFetch:
    def __init__(self):
        pass

    async def fetch_index_info(self) -> List[Dict]:
        pass

    async def fetch_index_bar(self, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None) -> Dict:
        pass

    async def fetch_stock_info(self) -> List[Dict]:
        pass

    async def fetch_stock_is_margin(self) -> Set[str]:
        pass

    async def fetch_stock_bar(self, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None) -> Dict:
        pass

    async def fetch_stock_index(self, index_date: Optional[date]) -> Dict[str, Dict]:
        pass

    async def fetch_stock_industry(self) -> List[Dict]:
        pass

    async def fetch_stock_industry_detail(self, code: Optional[str] = None,
                                    name: Optional[str] = None) -> List[Dict]:
        pass

    async def fetch_stock_industry_daily(self, code: str, name: Optional[str] = None,
                                   start: Optional[date] = None, end: Optional[date] = None) -> Dict:
        pass

    async def fetch_stock_concept(self) -> List[Dict]:
        pass

    async def fetch_stock_concept_detail(self, code: Optional[str] = None, name: Optional[str] = None) -> List[Dict]:
        pass

    async def fetch_stock_concept_daily(self, code: str, name: Optional[str] = None,
                                  start: Optional[date] = None, end: Optional[date] = None) -> Dict:
        pass

    async def fetch_stock_yjbb(self, year: int, season: int) -> List[Dict]:
        pass

    async def fetch_stock_margin(self, code: str, start: Optional[date] = None, end: Optional[date] = None) -> List[Dict]:
        pass

    async def fetch_stock_rt_quot(self, code: List[str]) -> Dict[str, Dict]:
        pass


class BlockStockFetch:
    def __init__(self):
        pass

    def fetch_index_info(self) -> List[Dict]:
        pass

    def fetch_index_bar(self, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None) -> Dict:
        pass

    def fetch_stock_info(self) -> List[Dict]:
        pass

    def fetch_stock_is_margin(self) -> Set[str]:
        pass

    def fetch_stock_bar(self, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None) -> Dict:
        pass

    def fetch_stock_index(self, index_date: Optional[date]) -> Dict[str, Dict]:
        pass

    def fetch_stock_industry(self) -> List[Dict]:
        pass

    def fetch_stock_industry_detail(self, code: Optional[str] = None,
                                    name: Optional[str] = None) -> List[Dict]:
        pass

    def fetch_stock_industry_daily(self, code: str, name: Optional[str] = None,
                                   start: Optional[date] = None, end: Optional[date] = None) -> Dict:
        pass

    def fetch_stock_concept(self) -> List[Dict]:
        pass

    def fetch_stock_concept_detail(self, code: Optional[str] = None, name: Optional[str] = None) -> List[Dict]:
        pass

    def fetch_stock_concept_daily(self, code: str, name: Optional[str] = None,
                                  start: Optional[date] = None, end: Optional[date] = None) -> Dict:
        pass

    def fetch_stock_yjbb(self, year: int, season: int) -> List[Dict]:
        pass

    def fetch_stock_margin(self, code: str, start: Optional[date] = None, end: Optional[date] = None) -> List[Dict]:
        pass

    def fetch_stock_rt_quot(self, code: List[str]) -> Dict[str, Dict]:
        pass
