from pywqfetch.fetch import BlockFetch, Fetch
from pywqfetch.pywqfetch import to_std_code


class BarFreq:
    """k线周期"""
    Min1 = 1
    """1分钟k线"""
    Min5 = 5
    """5分钟k线"""
    Min15 = 15
    """15分钟k线"""
    Min30 = 30
    """30分钟k线"""
    Min60 = 60
    """60分钟k线"""
    Daily = 101
    """日线"""
    Weekly = 102
    """周线"""
    Monthly = 103
    """月线"""
    LooseDaily = 1010
    """日线，在交易日没结束前，显示的是最新值，交易日结束后，同Daily"""


class MarketType:
    """市场品种"""
    Bond = 0
    """可转债"""
    Fund = 1
    """ETF基金"""
    Stock = 2
    """股票"""


rwq_fetch = Fetch()
"""非阻塞版本 :class:`~pywqfetch.fetch.Fetch` 全局实例"""


fetch_trade_date = rwq_fetch.fetch_trade_date
"""同 :func:`~pywqfetch.Fetch.fetch_trade_date`"""
fetch_next_trade_date = rwq_fetch.fetch_next_trade_date
"""同 :func:`~pywqfetch.Fetch.fetch_trade_date`"""
fetch_prev_trade_date = rwq_fetch.fetch_next_trade_date
"""同 :func:`~pywqfetch.Fetch.fetch_prev_trade_date`"""
fetch_is_trade_date = rwq_fetch.fetch_is_trade_date
"""同 :func:`~pywqfetch.Fetch.fetch_is_trade_date`"""

# bond
fetch_bond_info = rwq_fetch.fetch_bond_info
"""同 :func:`~pywqfetch.Fetch.fetch_bond_info`"""
fetch_bond_bar = rwq_fetch.fetch_bond_bar
"""同 :func:`~pywqfetch.Fetch.fetch_bond_bar`"""

# fund
fetch_fund_info = rwq_fetch.fetch_fund_info
"""同 :func:`~pywqfetch.Fetch.fetch_fund_info`"""
fetch_fund_net = rwq_fetch.fetch_fund_net
"""同 :func:`~pywqfetch.Fetch.fetch_fund_net`"""
fetch_fund_bar = rwq_fetch.fetch_fund_bar
"""同 :func:`~pywqfetch.Fetch.fetch_fund_bar`"""

# stock
fetch_index_info = rwq_fetch.fetch_index_info
"""同 :func:`~pywqfetch.Fetch.fetch_index_info`"""
fetch_index_bar = rwq_fetch.fetch_index_bar
"""同 :func:`~pywqfetch.Fetch.fetch_index_bar`"""
fetch_stock_info = rwq_fetch.fetch_stock_info
"""同 :func:`~pywqfetch.Fetch.fetch_stock_info`"""
fetch_stock_is_margin = rwq_fetch.fetch_stock_is_margin
"""同 :func:`~pywqfetch.Fetch.fetch_stock_is_margin`"""
fetch_stock_bar = rwq_fetch.fetch_stock_bar
"""同 :func:`~pywqfetch.Fetch.fetch_stock_bar`"""
fetch_stock_index = rwq_fetch.fetch_stock_index
"""同 :func:`~pywqfetch.Fetch.fetch_stock_index`"""
fetch_stock_industry = rwq_fetch.fetch_stock_industry
"""同 :func:`~pywqfetch.Fetch.fetch_stock_industry`"""
fetch_stock_industry_detail = rwq_fetch.fetch_stock_industry_detail
"""同 :func:`~pywqfetch.Fetch.fetch_stock_industry_detail`"""
fetch_stock_industry_daily = rwq_fetch.fetch_stock_industry_daily
"""同 :func:`~pywqfetch.Fetch.fetch_stock_industry_daily`"""
fetch_stock_concept = rwq_fetch.fetch_stock_concept
"""同 :func:`~pywqfetch.Fetch.fetch_stock_concept`"""
fetch_stock_concept_detail = rwq_fetch.fetch_stock_concept_detail
"""同 :func:`~pywqfetch.Fetch.fetch_stock_concept_detail`"""
fetch_stock_concept_daily = rwq_fetch.fetch_stock_concept_daily
"""同 :func:`~pywqfetch.Fetch.fetch_stock_concept_daily`"""
fetch_stock_yjbb = rwq_fetch.fetch_stock_yjbb
"""同 :func:`~pywqfetch.Fetch.fetch_stock_yjbb`"""
fetch_stock_margin = rwq_fetch.fetch_stock_margin
"""同 :func:`~pywqfetch.Fetch.fetch_stock_margin`"""
fetch_stock_hot_rank = rwq_fetch.fetch_stock_hot_rank
"""同 :func:`~pywqfetch.Fetch.fetch_stock_hot_rank`"""
fetch_stock_rt_quot = rwq_fetch.fetch_stock_rt_quot
"""同 :func:`~pywqfetch.Fetch.fetch_stock_rt_quot`"""


rwq_block_fetch = BlockFetch()
"""阻塞版本 :class:`~pywqfetch.fetch.BlockFetch` 全局实例"""


block_fetch_trade_date = rwq_block_fetch.fetch_trade_date
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_trade_date`"""
block_fetch_next_trade_date = rwq_block_fetch.fetch_next_trade_date
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_next_trade_date`"""
block_fetch_prev_trade_date = rwq_block_fetch.fetch_prev_trade_date
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_prev_trade_date`"""
block_fetch_is_trade_date = rwq_block_fetch.fetch_is_trade_date
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_is_trade_date`"""

# bond
block_fetch_bond_info = rwq_block_fetch.fetch_bond_info
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_bond_info`"""
block_fetch_bond_bar = rwq_block_fetch.fetch_bond_bar
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_bond_bar`"""

# fund
block_fetch_fund_info = rwq_block_fetch.fetch_fund_info
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_fund_info`"""
block_fetch_fund_net = rwq_block_fetch.fetch_fund_net
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_fund_net`"""
block_fetch_fund_bar = rwq_block_fetch.fetch_fund_bar
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_fund_bar`"""

# stock
block_fetch_index_info = rwq_block_fetch.fetch_index_info
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_index_info`"""
block_fetch_index_bar = rwq_block_fetch.fetch_index_bar
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_index_bar`"""
block_fetch_stock_info = rwq_block_fetch.fetch_stock_info
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_info`"""
block_fetch_stock_is_margin = rwq_block_fetch.fetch_stock_is_margin
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_is_margin`"""
block_fetch_stock_bar = rwq_block_fetch.fetch_stock_bar
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_bar`"""
block_fetch_stock_index = rwq_block_fetch.fetch_stock_index
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_index`"""
block_fetch_stock_industry = rwq_block_fetch.fetch_stock_industry
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_industry`"""
block_fetch_stock_industry_detail = rwq_block_fetch.fetch_stock_industry_detail
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_industry_detail`"""
block_fetch_stock_industry_daily = rwq_block_fetch.fetch_stock_industry_daily
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_industry_daily`"""
block_fetch_stock_concept = rwq_block_fetch.fetch_stock_concept
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_concept`"""
block_fetch_stock_concept_detail = rwq_block_fetch.fetch_stock_concept_detail
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_concept_detail`"""
block_fetch_stock_concept_daily = rwq_block_fetch.fetch_stock_concept_daily
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_concept_daily`"""
block_fetch_stock_yjbb = rwq_block_fetch.fetch_stock_yjbb
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_yjbb`"""
block_fetch_stock_margin = rwq_block_fetch.fetch_stock_margin
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_margin`"""
block_fetch_stock_hot_rank = rwq_block_fetch.fetch_stock_hot_rank
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_hot_rank`"""
block_fetch_stock_rt_quot = rwq_block_fetch.fetch_stock_rt_quot
"""阻塞版本，参考 :func:`~pywqfetch.Fetch.fetch_stock_rt_quot`"""
