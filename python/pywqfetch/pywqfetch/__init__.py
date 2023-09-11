from pywqfetch.fetch import BlockFetch, Fetch
from pywqfetch.pywqfetch import to_std_code


class BarFreq:
    Min1 = 1
    Min5 = 5
    Min15 = 15
    Min30 = 30
    Min60 = 60
    Daily = 101
    Weekly = 102
    Monthly = 103
    LooseDaily = 1010


class MarketType:
    Bond = 0
    Fund = 1
    Stock = 2



rwqfetch = Fetch() 
"""快捷非阻塞版本fetch"""


fetch_trade_date = rwqfetch.fetch_trade_date
fetch_next_trade_date = rwqfetch.fetch_next_trade_date
fetch_prev_trade_date = rwqfetch.fetch_prev_trade_date
fetch_is_trade_date = rwqfetch.fetch_is_trade_date

# bond
fetch_bond_info = rwqfetch.fetch_bond_info
fetch_bond_bar = rwqfetch.fetch_bond_bar

# fund
fetch_fund_info = rwqfetch.fetch_fund_info
fetch_fund_net = rwqfetch.fetch_fund_net
fetch_fund_bar = rwqfetch.fetch_fund_bar

# stock
fetch_index_info = rwqfetch.fetch_index_info
fetch_index_bar = rwqfetch.fetch_index_bar
fetch_stock_info = rwqfetch.fetch_stock_info
fetch_stock_is_margin = rwqfetch.fetch_stock_is_margin
fetch_stock_bar = rwqfetch.fetch_stock_bar
fetch_stock_index = rwqfetch.fetch_stock_index
fetch_stock_industry = rwqfetch.fetch_stock_industry
fetch_stock_industry_detail = rwqfetch.fetch_stock_industry_detail
fetch_stock_industry_daily = rwqfetch.fetch_stock_industry_daily
fetch_stock_concept = rwqfetch.fetch_stock_concept
fetch_stock_concept_detail = rwqfetch.fetch_stock_concept_detail
fetch_stock_concept_daily = rwqfetch.fetch_stock_concept_daily
fetch_stock_yjbb = rwqfetch.fetch_stock_yjbb
fetch_stock_margin = rwqfetch.fetch_stock_margin
fetch_stock_hot_rank = rwqfetch.fetch_stock_hot_rank
fetch_stock_rt_quot = rwqfetch.fetch_stock_rt_quot


my_block_fetch = BlockFetch()

block_fetch_trade_date = my_block_fetch.fetch_trade_date
block_fetch_next_trade_date = my_block_fetch.fetch_next_trade_date
block_fetch_prev_trade_date = my_block_fetch.fetch_prev_trade_date
block_fetch_is_trade_date = my_block_fetch.fetch_is_trade_date

# bond
block_fetch_bond_info = my_block_fetch.fetch_bond_info
block_fetch_bond_bar = my_block_fetch.fetch_bond_bar

# fund
block_fetch_fund_info = my_block_fetch.fetch_fund_info
block_fetch_fund_net = my_block_fetch.fetch_fund_net
block_fetch_fund_bar = my_block_fetch.fetch_fund_bar

# stock
block_fetch_index_info = my_block_fetch.fetch_index_info
block_fetch_index_bar = my_block_fetch.fetch_index_bar
block_fetch_stock_info = my_block_fetch.fetch_stock_info
block_fetch_stock_is_margin = my_block_fetch.fetch_stock_is_margin
block_fetch_stock_bar = my_block_fetch.fetch_stock_bar
block_fetch_stock_index = my_block_fetch.fetch_stock_index
block_fetch_stock_industry = my_block_fetch.fetch_stock_industry
block_fetch_stock_industry_detail = my_block_fetch.fetch_stock_industry_detail
block_fetch_stock_industry_daily = my_block_fetch.fetch_stock_industry_daily
block_fetch_stock_concept = my_block_fetch.fetch_stock_concept
block_fetch_stock_concept_detail = my_block_fetch.fetch_stock_concept_detail
block_fetch_stock_concept_daily = my_block_fetch.fetch_stock_concept_daily
block_fetch_stock_yjbb = my_block_fetch.fetch_stock_yjbb
block_fetch_stock_margin = my_block_fetch.fetch_stock_margin
block_fetch_stock_hot_rank = my_block_fetch.fetch_stock_hot_rank
block_fetch_stock_rt_quot = my_block_fetch.fetch_stock_rt_quot
