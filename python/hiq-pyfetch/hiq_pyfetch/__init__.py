from hiq_pyfetch.my_fetch import MyBlockFetch, MyFetch
from hiq_pyfetch.hiq_pyfetch import to_std_code


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


my_fetch = MyFetch()

fetch_trade_date = my_fetch.fetch_trade_date
fetch_next_trade_date = my_fetch.fetch_next_trade_date
fetch_prev_trade_date = my_fetch.fetch_prev_trade_date

# bond
fetch_bond_info = my_fetch.fetch_bond_info
fetch_bond_bar = my_fetch.fetch_bond_bar

# fund
fetch_fund_info = my_fetch.fetch_fund_info
fetch_fund_net = my_fetch.fetch_fund_net
fetch_fund_bar = my_fetch.fetch_fund_bar

# stock
fetch_index_info = my_fetch.fetch_index_info
fetch_index_bar = my_fetch.fetch_index_bar
fetch_stock_info = my_fetch.fetch_stock_info
fetch_stock_is_margin = my_fetch.fetch_stock_is_margin
fetch_stock_bar = my_fetch.fetch_stock_bar
fetch_stock_index = my_fetch.fetch_stock_index
fetch_stock_industry = my_fetch.fetch_stock_industry
fetch_stock_industry_detail = my_fetch.fetch_stock_industry_detail
fetch_stock_industry_daily = my_fetch.fetch_stock_industry_daily
fetch_stock_concept = my_fetch.fetch_stock_concept
fetch_stock_concept_detail = my_fetch.fetch_stock_concept_detail
fetch_stock_concept_daily = my_fetch.fetch_stock_concept_daily
fetch_stock_yjbb = my_fetch.fetch_stock_yjbb
fetch_stock_margin = my_fetch.fetch_stock_margin
fetch_stock_rt_quot = my_fetch.fetch_stock_rt_quot


my_block_fetch = MyBlockFetch()

block_fetch_trade_date = my_block_fetch.fetch_trade_date

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
block_fetch_stock_rt_quot = my_block_fetch.fetch_stock_rt_quot
