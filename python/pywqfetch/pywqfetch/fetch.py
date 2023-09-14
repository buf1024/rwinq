from datetime import date, datetime
from typing import List, Dict, Union, Optional, Set

from pywqfetch.pywqfetch import BlockBondFetch, BlockFundFetch, BlockStockFetch, BondFetch, FundFetch, StockFetch, block_fetch_next_trade_date, block_fetch_prev_trade_date, fetch_next_trade_date, fetch_is_trade_date, block_fetch_is_trade_date, fetch_prev_trade_date, fetch_trade_date, block_fetch_trade_date, fetch_rt_quot, block_fetch_rt_quot, calc_chip_dist as _calc_chip_dist, calc_cost as _calc_cost, calc_winner as _calc_winner


import pandas as pd


def calc_chip_dist(*, data: pd.DataFrame, ac: int = 1, chip_dist: Dict = None) -> Dict:
    data = data.to_dict('records')
    return _calc_chip_dist(data=data, ac=ac, chip_dist=chip_dist)


def calc_winner(*, chip_dist: Dict, data: pd.DataFrame = None, price: float = None) -> Dict:
    data = data.to_dict('records') if data is not None else None
    return _calc_winner(chip_dist=chip_dist, data=data, price=price)


def calc_cost(*, chip_dist: Dict, ratio: int) -> Dict:
    return _calc_cost(chip_dist=chip_dist, ratio=ratio)


async def _fetch_rt_quot(*, code: List[str], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
    data = await fetch_rt_quot(code)
    if to_frame and data != None and len(data) > 0:
        data = pd.DataFrame([v for v in data.values()])
    return data


async def _block_fetch_rt_quot(*, code: List[str], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
    data = block_fetch_rt_quot(code)
    if to_frame and data != None and len(data) > 0:
        data = pd.DataFrame([v for v in data.values()])
    return data


class Fetch:
    def __init__(self):
        self.bond_fetch = BondFetch()
        self.fund_fetch = FundFetch()
        self.stock_fetch = StockFetch()

    @staticmethod
    def _to_dataframe(to_frame, data):
        if to_frame and data is not None:
            return pd.DataFrame(data)
        return data

    @staticmethod
    async def fetch_trade_date(to_frame=True) -> Union[Set[int], pd.DataFrame]:
        data = await fetch_trade_date()
        if to_frame:
            data = pd.DataFrame(data)
        data.columns = ['trade_date']
        return data

    @staticmethod
    async def fetch_next_trade_date(d: Union[date, datetime]) -> date:
        data = await fetch_next_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    async def fetch_prev_trade_date(d: Union[date, datetime]) -> date:
        data = await fetch_prev_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    async def fetch_is_trade_date(d: Union[date, datetime]) -> bool:
        return await fetch_is_trade_date(d)

    @staticmethod
    async def fetch_rt_quot(*, code: List[str], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        return await _fetch_rt_quot(code=code, to_frame=to_frame)

    # bond
    async def fetch_bond_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.bond_fetch.fetch_bond_info())

    async def fetch_bond_bar(self, *, code: str, name: str,
                             stock_code: str, stock_name: str,
                             freq: Optional[int] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             skip_rt: bool = True,
                             to_frame=True, ) -> Dict:
        data = await self.bond_fetch.fetch_bond_bar(code=code, name=name,
                                                    stock_code=stock_code, stock_name=stock_name,
                                                    freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    # fund
    async def fetch_fund_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.fund_fetch.fetch_fund_info())

    async def fetch_fund_net(self, *, code: str, name: Optional[str] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.fund_fetch.fetch_fund_net(code=code, name=name,
                                                                       start=start, end=end))

    async def fetch_fund_bar(self, *, code: str, name: Optional[str] = None,
                             freq: Optional[int] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             skip_rt: bool = True,
                             to_frame=True) -> Dict:
        data = await self.fund_fetch.fetch_fund_bar(code=code, name=name,
                                                    freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    # stock
    async def fetch_index_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_index_info())

    async def fetch_index_bar(self, *, code: str, name: Optional[str] = None,
                              freq: Optional[int] = None,
                              start: Optional[date] = None, end: Optional[date] = None, skip_rt: bool = True, to_frame=True) -> Dict:
        data = await self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                      freq=freq, start=start, end=end,
                                                      skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    async def fetch_stock_info(self, *, market: int = None, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_info(market))

    async def fetch_stock_is_margin(self, *, to_frame=True) -> Union[Set[str], pd.DataFrame]:
        data = await self.stock_fetch.fetch_stock_is_margin()
        if to_frame:
            data = pd.DataFrame(data)
            data.columns = ['code']
        return data

    async def fetch_stock_bar(self, *, code: str, name: Optional[str] = None,
                              freq: Optional[int] = None,
                              start: Optional[date] = None, end: Optional[date] = None,
                              skip_rt: bool = True,
                              to_frame=True) -> Union[Dict, pd.DataFrame]:
        data = await self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                      freq=freq, start=start, end=end,
                                                      skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    async def fetch_stock_index(self, *, index_date: Optional[date] = None, to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        data = await self.stock_fetch.fetch_stock_index(index_date)
        if to_frame:
            data = pd.DataFrame(list(data.values()))
        return data

    async def fetch_stock_industry(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_industry())

    async def fetch_stock_industry_detail(self, *, code: Optional[str] = None,
                                          name: Optional[str] = None,
                                          to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_industry_detail(code, name))

    async def fetch_stock_industry_daily(self, *, code: str, name: Optional[str] = None,
                                         start: Optional[date] = None, end: Optional[date] = None,
                                         skip_rt: bool = True,
                                         to_frame=True) -> Union[Dict, pd.DataFrame]:

        data = await self.stock_fetch.fetch_stock_industry_daily(code=code, name=name,
                                                                 start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    async def fetch_stock_concept(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_concept())

    async def fetch_stock_concept_detail(self, *, code: Optional[str] = None, name: Optional[str] = None,
                                         to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_concept_detail(code, name))

    async def fetch_stock_concept_daily(self, *, code: str, name: Optional[str] = None,
                                        start: Optional[date] = None, end: Optional[date] = None,
                                        skip_rt: bool = True,
                                        to_frame=True) -> Union[Dict, pd.DataFrame]:
        data = await self.stock_fetch.fetch_stock_industry_daily(code=code, name=name,
                                                                 start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    async def fetch_stock_yjbb(self, *, year: int, season: int,
                               to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_yjbb(year, season))

    async def fetch_stock_margin(self, *, code: str, start: Optional[date] = None, end: Optional[date] = None,
                                 to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_margin(code, start, end))

    async def fetch_stock_hot_rank(self, *, code: Union[str, list],
                                   to_frame=True) -> Union[Dict, pd.DataFrame]:
        codes = code
        if type(code) == type(''):
            codes = [code]
        datas = []
        for code in codes:
            data = await self.stock_fetch.fetch_stock_hot_rank(code=code)
            datas.append(data)
        return self._to_dataframe(to_frame, data=datas)


class BlockFetch:
    def __init__(self):
        self.bond_fetch = BlockBondFetch()
        self.fund_fetch = BlockFundFetch()
        self.stock_fetch = BlockStockFetch()

    @staticmethod
    def _to_dataframe(to_frame, data):
        if to_frame and data is not None:
            return pd.DataFrame(data)
        return data

    @staticmethod
    def fetch_trade_date(to_frame=True) -> Union[Set[int], pd.DataFrame]:
        data = block_fetch_trade_date()
        if to_frame:
            data = pd.DataFrame(data)
        data.columns = ['trade_date']
        return data

    @staticmethod
    def fetch_next_trade_date(d: Union[date, datetime]) -> date:
        data = block_fetch_next_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    def fetch_prev_trade_date(d: Union[date, datetime]) -> date:
        data = block_fetch_prev_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    def fetch_is_trade_date(d) -> bool:
        return block_fetch_is_trade_date(d)

    @staticmethod
    def fetch_rt_quot(*, code: List[str], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        return _block_fetch_rt_quot(code=code, to_frame=to_frame)

    # bond
    def fetch_bond_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.bond_fetch.fetch_bond_info())

    def fetch_bond_bar(self, *, code: str, name: str,
                       stock_code: str, stock_name: str,
                       freq: Optional[int] = None,
                       start: Optional[date] = None, end: Optional[date] = None,
                       skip_rt: bool = True,
                       to_frame=True, ) -> Dict:
        data = self.bond_fetch.fetch_bond_bar(code=code, name=name,
                                              stock_code=stock_code, stock_name=stock_name,
                                              freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    # fund
    def fetch_fund_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.fund_fetch.fetch_fund_info())

    def fetch_fund_net(self, *, code: str, name: Optional[str] = None,
                       start: Optional[date] = None, end: Optional[date] = None,
                       to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.fund_fetch.fetch_fund_net(code=code, name=name,
                                                                 start=start, end=end))

    def fetch_fund_bar(self, *, code: str, name: Optional[str] = None,
                       freq: Optional[int] = None,
                       start: Optional[date] = None, end: Optional[date] = None,
                       skip_rt: bool = True,
                       to_frame=True) -> Dict:
        data = self.fund_fetch.fetch_fund_bar(code=code, name=name,
                                              freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    # stock
    def fetch_index_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_index_info())

    def fetch_index_bar(self, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None,
                        skip_rt: bool = True,
                        to_frame=True) -> Union[Dict, pd.DataFrame]:
        data = self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    def fetch_stock_info(self, *, market: int = None,  to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_info(market))

    def fetch_stock_is_margin(self, *, to_frame=True) -> Union[Set[str], pd.DataFrame]:
        data = self.stock_fetch.fetch_stock_is_margin()
        if to_frame:
            data = pd.DataFrame(data)
            data.columns = ['code']
        return data

    def fetch_stock_bar(self, *, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None,
                        skip_rt: bool = True,
                        to_frame=True) -> Union[Dict, pd.DataFrame]:
        data = self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    def fetch_stock_index(self, *, index_date=None, to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        data = self.stock_fetch.fetch_stock_index(index_date)
        if to_frame:
            data = pd.DataFrame(list(data.values()))
        return data

    def fetch_stock_industry(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_industry())

    def fetch_stock_industry_detail(self, *, code: Optional[str] = None,
                                    name: Optional[str] = None,
                                    to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_industry_detail(code, name))

    def fetch_stock_industry_daily(self, code: str, name: Optional[str] = None,
                                   start: Optional[date] = None, end: Optional[date] = None,
                                   skip_rt: bool = True,
                                   to_frame=True) -> Union[Dict, pd.DataFrame]:
        data = self.stock_fetch.fetch_stock_industry_daily(code=code, name=name,
                                                           start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    def fetch_stock_concept(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_concept())

    def fetch_stock_concept_detail(self, *, code: Optional[str] = None, name: Optional[str] = None,
                                   to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_concept_detail(code, name))

    def fetch_stock_concept_daily(self, *, code: str, name: Optional[str] = None,
                                  start: Optional[date] = None, end: Optional[date] = None,
                                  skip_rt: bool = True,
                                  to_frame=True) -> Union[Dict, pd.DataFrame]:
        data = self.stock_fetch.fetch_stock_industry_daily(code=code, name=name,
                                                           start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    def fetch_stock_yjbb(self, *, year: int, season: int,
                         to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_yjbb(year, season))

    def fetch_stock_margin(self, *, code: str, start: Optional[date] = None, end: Optional[date] = None,
                           to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_margin(code, start, end))

    def fetch_stock_hot_rank(self, *, code: Union[str, list],
                             to_frame=True) -> Union[Dict, pd.DataFrame]:
        codes = code
        if type(code) == type(''):
            codes = [code]
        datas = []
        for code in codes:
            data = self.stock_fetch.fetch_stock_hot_rank(code=code)
            datas.append(data)
        return self._to_dataframe(to_frame, data=datas)
