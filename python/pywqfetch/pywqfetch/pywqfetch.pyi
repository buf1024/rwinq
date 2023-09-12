from typing import List, Dict, Optional, Set
from datetime import date


async def fetch_trade_date() -> Set[int]:
    """Rust封装的函数，除了返回会原始类外，同 :func:`~pywqfetch.fetch.Fetch.fetch_trade_date` """
    pass


def block_fetch_trade_date() -> Set[int]:
    """Rust封装的函数，阻塞版本，除了返回会原始类外，同 :func:`~pywqfetch.fetch.Fetch.block_fetch_trade_date` """
    pass


async def fetch_next_trade_date(d: date) -> int:
    """Rust封装的函数，除了返回会原始类外，同 :func:`~pywqfetch.fetch.Fetch.fetch_next_trade_date` """
    pass


def block_fetch_next_trade_date(d: date) -> int:
    """Rust封装的函数，阻塞版本，除了返回会原始类外，同 :func:`~pywqfetch.fetch.Fetch.block_fetch_next_trade_date` """
    pass


async def fetch_prev_trade_date(d: date) -> int:
    """Rust封装的函数，除了返回会原始类外，同 :func:`~pywqfetch.fetch.Fetch.fetch_prev_trade_date` """
    pass


def block_fetch_prev_trade_date(d: date) -> int:
    """Rust封装的函数，阻塞版本，除了返回会原始类外，同 :func:`~pywqfetch.fetch.Fetch.block_fetch_prev_trade_date` """
    pass


async def fetch_is_trade_date(d: date) -> bool:
    """Rust封装的函数，除了返回会原始类外，同 :func:`~pywqfetch.fetch.Fetch.fetch_is_trade_date` """
    pass


def block_fetch_is_trade_date(d: date) -> bool:
    """Rust封装的函数，阻塞版本，除了返回会原始类外，同 :func:`~pywqfetch.fetch.Fetch.block_fetch_is_trade_date` """
    pass


def to_std_code(typ: int, code: str) -> str:
    """转化为内部使用前缀为sz/sh/bj的代码

    Args:
        typ (int): 市场品种 参考 :class:`~pywqfetch.MarketType`
        code (str): 原始的代码

    Returns:
        str: 前缀为sz/sh/bj的代码
    """
    pass


class BondFetch:
    """Rust封装的类，已经集成在 :class:`~pywqfetch.fetch.Fetch`"""

    def __init__(self):
        pass

    async def fetch_bond_info(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_bond_info`"""
        pass

    async def fetch_bond_bar(self, code: str, name: str,
                             stock_code: str, stock_name: str,
                             freq: Optional[int],
                             start: Optional[date],
                             end: Optional[date],
                             skip_rt: bool) -> Dict:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_bond_bar`"""
        pass


class BlockBondFetch:
    """Rust封装的类，阻塞版本已经集成在 :class:`~pywqfetch.fetch.BlockFetch`"""

    def __init__(self):
        pass

    async def fetch_bond_info(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_bond_info`"""
        pass

    async def fetch_bond_bar(self, code: str, name: str,
                             stock_code: str, stock_name: str,
                             freq: Optional[int],
                             start: Optional[date],
                             end: Optional[date],
                             skip_rt: bool) -> Dict:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_bond_bar`"""
        pass


class FundFetch:
    """Rust封装的类 :class:`~pywqfetch.fetch.FundFetch`"""

    def __init__(self):
        pass

    async def fetch_fund_info(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_bond_bar`"""
        pass

    async def fetch_fund_net(self, code: str, name: Optional[str],
                             start: Optional[date], end: Optional[date]) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_bond_bar`"""
        pass

    async def fetch_fund_bar(self, code: str, name: Optional[str],
                             freq: Optional[int],
                             start: Optional[date],
                             end: Optional[date],
                             skip_rt: bool) -> Dict:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_bond_bar`"""
        pass


class BlockFundFetch:
    """Rust封装的类，阻塞版本已经集成在 :class:`~pywqfetch.fetch.BlockFetch`"""

    def __init__(self):
        pass

    def fetch_fund_info(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_fund_info`"""
        pass

    def fetch_fund_net(self, code: str, name: Optional[str],
                       start: Optional[date], end: Optional[date]) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_fund_net`"""
        pass

    def fetch_fund_bar(self, code: str, name: Optional[str],
                       freq: Optional[int],
                       start: Optional[date],
                       end: Optional[date],
                       skip_rt: bool) -> Dict:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_fund_bar`"""
        pass


class StockFetch:
    """Rust封装的类 :class:`~pywqfetch.fetch.Fetch`"""

    def __init__(self):
        pass

    async def fetch_index_info(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_index_info`"""
        pass

    async def fetch_index_bar(self, code: str, name: Optional[str] = None,
                              freq: Optional[int] = None,
                              start: Optional[date] = None,
                              end: Optional[date] = None,
                              skip_rt: bool = True) -> Dict:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_index_bar`"""
        pass

    async def fetch_stock_info(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_info`"""
        pass

    async def fetch_stock_is_margin(self) -> Set[str]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_is_margin`"""
        pass

    async def fetch_stock_bar(self, code: str, name: Optional[str] = None,
                              freq: Optional[int] = None,
                              start: Optional[date] = None,
                              end: Optional[date] = None,
                              skip_rt: bool = True) -> Dict:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_bar`"""
        pass

    async def fetch_stock_index(self, index_date: Optional[date]) -> Dict[str, Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_index`"""
        pass

    async def fetch_stock_industry(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_industry`"""
        pass

    async def fetch_stock_industry_detail(self, code: Optional[str] = None,
                                          name: Optional[str] = None) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_industry_detail`"""
        pass

    async def fetch_stock_industry_daily(self, code: str, name: Optional[str] = None,
                                         start: Optional[date] = None,
                                         end: Optional[date] = None,
                                         skip_rt: bool = True) -> Dict:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_industry_daily`"""
        pass

    async def fetch_stock_concept(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_concept`"""
        pass

    async def fetch_stock_concept_detail(self, code: Optional[str] = None, name: Optional[str] = None) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_concept_detail`"""
        pass

    async def fetch_stock_concept_daily(self, code: str, name: Optional[str] = None,
                                        start: Optional[date] = None,
                                        end: Optional[date] = None,
                                        skip_rt: bool = True) -> Dict:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_concept_daily`"""
        pass

    async def fetch_stock_yjbb(self, year: int, season: int) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_yjbb`"""
        pass

    async def fetch_stock_margin(self, code: str, start: Optional[date] = None, end: Optional[date] = None) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_margin`"""
        pass

    async def fetch_stock_hot_rank(self, code: str) -> Dict:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_hot_rank`"""
        pass

    async def fetch_stock_rt_quot(self, code: List[str]) -> Dict[str, Dict]:
        """参考 :class:`~pywqfetch.fetch.Fetch.fetch_stock_rt_quot`"""
        pass


class BlockStockFetch:
    """Rust封装的类，阻塞版本已经集成在 :class:`~pywqfetch.fetch.BlockFetch`"""

    def __init__(self):
        pass

    def fetch_index_info(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_index_info`"""

        pass

    def fetch_index_bar(self, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None,
                        skip_rt: bool = True) -> Dict:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_index_bar`"""
        pass

    def fetch_stock_info(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_info`"""
        pass

    def fetch_stock_is_margin(self) -> Set[str]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_is_margin`"""
        pass

    def fetch_stock_bar(self, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None,
                        skip_rt: bool = True) -> Dict:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_bar`"""
        pass

    def fetch_stock_index(self, index_date: Optional[date]) -> Dict[str, Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_index`"""
        pass

    def fetch_stock_industry(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_industry`"""
        pass

    def fetch_stock_industry_detail(self, code: Optional[str] = None,
                                    name: Optional[str] = None) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_industry_detail`"""
        pass

    def fetch_stock_industry_daily(self, code: str, name: Optional[str] = None,
                                   start: Optional[date] = None, end: Optional[date] = None,
                                   skip_rt: bool = True) -> Dict:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_industry_daily`"""
        pass

    def fetch_stock_concept(self) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_concept`"""
        pass

    def fetch_stock_concept_detail(self, code: Optional[str] = None, name: Optional[str] = None) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_concept_detail`"""
        pass

    def fetch_stock_concept_daily(self, code: str, name: Optional[str] = None,
                                  start: Optional[date] = None, end: Optional[date] = None,
                                  skip_rt: bool = True) -> Dict:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_concept_daily`"""
        pass

    def fetch_stock_yjbb(self, year: int, season: int) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_yjbb`"""
        pass

    def fetch_stock_margin(self, code: str, start: Optional[date] = None, end: Optional[date] = None) -> List[Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_margin`"""
        pass

    def fetch_stock_hot_rank(self, code: str) -> Dict:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_hot_rank`"""
        pass

    def fetch_stock_rt_quot(self, code: List[str]) -> Dict[str, Dict]:
        """参考 :class:`~pywqfetch.fetch.BlockFetch.fetch_stock_rt_quot`"""
        pass
