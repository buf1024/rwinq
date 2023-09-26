from datetime import date, datetime
from typing import List, Dict, Union, Optional, Set


import pywqfetch.pywqfetch as wqfetch
from pywqfetch.pywqfetch import to_std_code


import pandas as pd


def calc_chip_dist(*, data: pd.DataFrame, ac: int = 1, chip_dist: Dict = None) -> Dict:
    data = data.to_dict('records')
    return wqfetch.calc_chip_dist(data=data, ac=ac, chip_dist=chip_dist)


def calc_winner(*, chip_dist: Dict, data: pd.DataFrame = None, price: float = None) -> Dict:
    data = data.to_dict('records') if data is not None else None
    return wqfetch.calc_winner(chip_dist=chip_dist, data=data, price=price)


def calc_cost(*, chip_dist: Dict, ratio: int) -> Dict:
    return wqfetch.calc_cost(chip_dist=chip_dist, ratio=ratio)


def _str_to_datetime(d: str) -> datetime:
    nd = None
    for fmt in ['%Y%m%d', '%Y-%m-%d']:
        try:
            nd = datetime.strptime(d, fmt)
            return nd
        except ValueError:
            pass
    if nd == None:
        raise Exception('date format invalid: {}'.format(d))


def _to_dataframe(to_frame, data):
    if to_frame and data is not None:
        data = pd.DataFrame(data)
    return data


async def fetch_trade_date(to_frame=True) -> Union[Set[int], pd.DataFrame]:
    data = await wqfetch.fetch_trade_date()
    if to_frame:
        data = pd.DataFrame(data)
    data.columns = ['trade_date']
    return data


def block_fetch_trade_date(to_frame=True) -> Union[Set[int], pd.DataFrame]:
    data = wqfetch.block_fetch_trade_date()
    if to_frame:
        data = pd.DataFrame(data)
    data.columns = ['trade_date']
    return data


async def fetch_next_trade_date(d: Union[date, datetime, str]) -> date:
    d = _str_to_datetime(d) if type(d) == type('') else d
    data = await fetch_next_trade_date(d)
    return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()


def block_fetch_next_trade_date(d: Union[date, datetime, str]) -> date:
    d = _str_to_datetime(d) if type(d) == type('') else d
    data = wqfetch.block_fetch_next_trade_date(d)
    return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()


async def fetch_prev_trade_date(d: Union[date, datetime, str]) -> date:
    d = _str_to_datetime(d) if type(d) == type('') else d
    data = await fetch_prev_trade_date(d)
    return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()


def block_fetch_prev_trade_date(d: Union[date, datetime, str]) -> date:
    d = _str_to_datetime(d) if type(d) == type('') else d
    data = wqfetch.block_fetch_prev_trade_date(d)
    return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()


async def fetch_is_trade_date(d: Union[date, datetime, str]) -> bool:
    d = _str_to_datetime(d) if type(d) == type('') else d
    return await fetch_is_trade_date(d)


def block_fetch_is_trade_date(d: Union[date, datetime, str]) -> bool:
    d = _str_to_datetime(d) if type(d) == type('') else d
    return wqfetch.block_fetch_is_trade_date(d)


async def fetch_rt_quot(*, code: Union[str, List[str]], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
    if type(code) == type(''):
        code = [code]
    data = await wqfetch.fetch_rt_quot(code)
    if to_frame and data != None and len(data) > 0:
        data = pd.DataFrame([v for v in data.values()])
    return data


def block_fetch_rt_quot(*, code: Union[str, List[str]], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
    if type(code) == type(''):
        code = [code]
    data = wqfetch.block_fetch_rt_quot(code)
    if to_frame and data != None and len(data) > 0:
        data = pd.DataFrame([v for v in data.values()])
    return data

# bond


async def fetch_bond_info(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_bond_info())


async def fetch_bond_bar(*, code: str, name: str,
                         stock_code: str, stock_name: str,
                         freq: Optional[int] = None,
                         start: Optional[date] = None, end: Optional[date] = None,
                         skip_rt: bool = True,
                         to_frame=True, ) -> Dict:
    data = await wqfetch.fetch_bond_bar(code=code, name=name,
                                        stock_code=stock_code, stock_name=stock_name,
                                        freq=freq, start=start, end=end, skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


def block_fetch_bond_bar(*, code: str, name: str,
                         stock_code: str, stock_name: str,
                         freq: Optional[int] = None,
                         start: Optional[date] = None, end: Optional[date] = None,
                         skip_rt: bool = True,
                         to_frame=True, ) -> Dict:
    data = wqfetch.block_fetch_bond_bar(code=code, name=name,
                                        stock_code=stock_code, stock_name=stock_name,
                                        freq=freq, start=start, end=end, skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data

# fund


async def fetch_fund_info(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_fund_info())


def block_fetch_fund_info(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_fund_info())


async def fetch_fund_net(*, code: str, name: Optional[str] = None,
                         start: Optional[date] = None, end: Optional[date] = None,
                         to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_fund_net(code=code, name=name,
                                                      start=start, end=end))


def block_fetch_fund_net(*, code: str, name: Optional[str] = None,
                         start: Optional[date] = None, end: Optional[date] = None,
                         to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_fund_net(code=code, name=name,
                                                      start=start, end=end))


async def fetch_fund_bar(*, code: str, name: Optional[str] = None,
                         freq: Optional[int] = None,
                         start: Optional[date] = None, end: Optional[date] = None,
                         skip_rt: bool = True,
                         to_frame=True) -> Dict:
    data = await wqfetch.fetch_fund_bar(code=code, name=name,
                                        freq=freq, start=start, end=end, skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


def block_fetch_fund_bar(*, code: str, name: Optional[str] = None,
                         freq: Optional[int] = None,
                         start: Optional[date] = None, end: Optional[date] = None,
                         skip_rt: bool = True,
                         to_frame=True) -> Dict:
    data = wqfetch.block_fetch_fund_bar(code=code, name=name,
                                        freq=freq, start=start, end=end, skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data

    # stock


async def fetch_index_info(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_index_info())


def block_fetch_index_info(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_index_info())


async def fetch_index_bar(*, code: str, name: Optional[str] = None,
                          freq: Optional[int] = None,
                          start: Optional[date] = None, end: Optional[date] = None, skip_rt: bool = True, to_frame=True) -> Dict:
    data = await wqfetch.fetch_stock_bar(code=code, name=name,
                                         freq=freq, start=start, end=end,
                                         skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


def block_fetch_index_bar(*, code: str, name: Optional[str] = None,
                          freq: Optional[int] = None,
                          start: Optional[date] = None, end: Optional[date] = None, skip_rt: bool = True, to_frame=True) -> Dict:
    data = wqfetch.block_fetch_stock_bar(code=code, name=name,
                                         freq=freq, start=start, end=end,
                                         skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


async def fetch_stock_info(*, market: int = None, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_stock_info(market))


def block_fetch_stock_info(*, market: int = None, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_stock_info(market))


async def fetch_stock_is_margin(*, to_frame=True) -> Union[Set[str], pd.DataFrame]:
    data = await wqfetch.fetch_stock_is_margin()
    if to_frame:
        data = pd.DataFrame(data)
        data.columns = ['code']
    return data


def block_fetch_stock_is_margin(*, to_frame=True) -> Union[Set[str], pd.DataFrame]:
    data = wqfetch.block_fetch_stock_is_margin()
    if to_frame:
        data = pd.DataFrame(data)
        data.columns = ['code']
    return data


async def fetch_stock_bar(*, code: str, name: Optional[str] = None,
                          freq: Optional[int] = None,
                          start: Optional[date] = None, end: Optional[date] = None,
                          skip_rt: bool = True,
                          to_frame=True) -> Union[Dict, pd.DataFrame]:
    data = await wqfetch.fetch_stock_bar(code=code, name=name,
                                         freq=freq, start=start, end=end,
                                         skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


def block_fetch_stock_bar(*, code: str, name: Optional[str] = None,
                          freq: Optional[int] = None,
                          start: Optional[date] = None, end: Optional[date] = None,
                          skip_rt: bool = True,
                          to_frame=True) -> Union[Dict, pd.DataFrame]:
    data = wqfetch.block_fetch_stock_bar(code=code, name=name,
                                         freq=freq, start=start, end=end,
                                         skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


async def fetch_stock_index(*, index_date: Optional[date] = None, to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
    data = await wqfetch.fetch_stock_index(index_date)
    if to_frame:
        data = pd.DataFrame(list(data.values()))
    return data


def block_fetch_stock_index(*, index_date: Optional[date] = None, to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
    data = wqfetch.block_fetch_stock_index(index_date)
    if to_frame:
        data = pd.DataFrame(list(data.values()))
    return data


async def fetch_stock_industry(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_stock_industry())


def block_fetch_stock_industry(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_stock_industry())


async def fetch_stock_industry_detail(*, code: Optional[str] = None,
                                      name: Optional[str] = None,
                                      to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_stock_industry_detail(code, name))


def block_fetch_stock_industry_detail(*, code: Optional[str] = None,
                                      name: Optional[str] = None,
                                      to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_stock_industry_detail(code, name))


async def fetch_stock_industry_daily(*, code: str, name: Optional[str] = None,
                                     start: Optional[date] = None, end: Optional[date] = None,
                                     skip_rt: bool = True,
                                     to_frame=True) -> Union[Dict, pd.DataFrame]:

    data = await wqfetch.fetch_stock_industry_daily(code=code, name=name,
                                                    start=start, end=end, skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


def block_fetch_stock_industry_daily(*, code: str, name: Optional[str] = None,
                                     start: Optional[date] = None, end: Optional[date] = None,
                                     skip_rt: bool = True,
                                     to_frame=True) -> Union[Dict, pd.DataFrame]:

    data = wqfetch.block_fetch_stock_industry_daily(code=code, name=name,
                                                    start=start, end=end, skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


async def fetch_stock_concept(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_stock_concept())


def block_fetch_stock_concept(*, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_stock_concept())


async def fetch_stock_concept_detail(*, code: Optional[str] = None, name: Optional[str] = None,
                                     to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_stock_concept_detail(code, name))


def block_fetch_stock_concept_detail(*, code: Optional[str] = None, name: Optional[str] = None,
                                     to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_stock_concept_detail(code, name))


async def fetch_stock_concept_daily(*, code: str, name: Optional[str] = None,
                                    start: Optional[date] = None, end: Optional[date] = None,
                                    skip_rt: bool = True,
                                    to_frame=True) -> Union[Dict, pd.DataFrame]:
    data = await wqfetch.fetch_stock_industry_daily(code=code, name=name,
                                                    start=start, end=end, skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


def block_fetch_stock_concept_daily(*, code: str, name: Optional[str] = None,
                                    start: Optional[date] = None, end: Optional[date] = None,
                                    skip_rt: bool = True,
                                    to_frame=True) -> Union[Dict, pd.DataFrame]:
    data = wqfetch.block_fetch_stock_industry_daily(code=code, name=name,
                                                    start=start, end=end, skip_rt=skip_rt)
    data['bars'] = _to_dataframe(to_frame, data['bars'])
    return data


async def fetch_stock_yjbb(*, year: int, season: int,
                           to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_stock_yjbb(year, season))


def block_fetch_stock_yjbb(*, year: int, season: int,
                           to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_stock_yjbb(year, season))


async def fetch_stock_margin(*, code: str, start: Optional[date] = None, end: Optional[date] = None,
                             to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         await wqfetch.fetch_stock_margin(code, start, end))


def block_fetch_stock_margin(*, code: str, start: Optional[date] = None, end: Optional[date] = None,
                             to_frame=True) -> Union[List[Dict], pd.DataFrame]:
    return _to_dataframe(to_frame,
                         wqfetch.block_fetch_stock_margin(code, start, end))


async def fetch_stock_hot_rank(*, code: Union[str, list],
                               to_frame=True) -> Union[Dict, pd.DataFrame]:
    codes = code
    if type(code) == type(''):
        codes = [code]
    datas = []
    for code in codes:
        data = await wqfetch.fetch_stock_hot_rank(code=code)
        datas.append(data)
    return _to_dataframe(to_frame, data=datas)


def block_fetch_stock_hot_rank(*, code: Union[str, list],
                               to_frame=True) -> Union[Dict, pd.DataFrame]:
    codes = code
    if type(code) == type(''):
        codes = [code]
    datas = []
    for code in codes:
        data = wqfetch.block_fetch_stock_hot_rank(code=code)
        datas.append(data)
    return _to_dataframe(to_frame, data=datas)


async def fetch_stock_comment(*, code: Optional[Union[List[str], str]] = None,
                              to_frame=True) -> Union[Dict, pd.DataFrame]:
    codes = code
    if type(code) == type(''):
        codes = [code]
    return _to_dataframe(to_frame, await wqfetch.fetch_stock_comment(codes=codes))


def block_fetch_stock_comment(*, code: Optional[Union[List[str], str]] = None,
                              to_frame=True) -> Union[Dict, pd.DataFrame]:
    codes = code
    if type(code) == type(''):
        codes = [code]
    return _to_dataframe(to_frame,  wqfetch.block_fetch_stock_comment(codes=codes))


async def fetch_stock_comment_his(code: str,
                                  to_frame=True) -> Dict:
    return _to_dataframe(to_frame, await wqfetch.fetch_stock_comment_his(code=code))


def block_fetch_stock_comment_his(code: str,
                                  to_frame=True) -> Dict:
    return _to_dataframe(to_frame,  wqfetch.block_fetch_stock_comment_his(code=code))
