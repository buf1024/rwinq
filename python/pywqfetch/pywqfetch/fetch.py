from datetime import date, datetime
from typing import List, Dict, Union, Optional, Set

from pywqfetch.pywqfetch import BlockBondFetch, BlockFundFetch, BlockStockFetch, BondFetch, FundFetch, StockFetch, block_fetch_next_trade_date, block_fetch_prev_trade_date, fetch_next_trade_date, fetch_is_trade_date, block_fetch_is_trade_date, fetch_prev_trade_date, fetch_trade_date, block_fetch_trade_date

import pandas as pd


class Fetch:
    """
    原始Rust包装类的汇聚类，包括债券，股票已经基金三类，通常使用此类来获取数据。

    参考: 
    :class:`~pywqfetch.pywqfetch.BondFetch`
    :class:`~pywqfetch.pywqfetch.FundFetch`
    :class:`~pywqfetch.pywqfetch.StockFetch`
    """

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
        """获取交易日期

        Args:
            to_frame (bool, optional): 结果是否转换为`pd.DataFrame`格式。 默认: True。

        Returns:
            Union[Set[int], pd.DataFrame]: to_frame 为 False时，返回Set， 否则返回`pd.DataFrame`

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_trade_date(to_frame=True)
                trade_date
            0       20120104
            ...          ...
            8068    20021231

            [8069 rows x 1 columns]
        """
        data = await fetch_trade_date()
        if to_frame:
            data = pd.DataFrame(data)
        data.columns = ['trade_date']
        return data

    @staticmethod
    async def fetch_next_trade_date(d: Union[date, datetime]) -> date:
        """获取某日的下一个交易日

        Args:
            d (Union[date, datetime]): 某时间

        Returns:
            date: 下一个交易日
        """
        data = await fetch_next_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    async def fetch_prev_trade_date(d: Union[date, datetime]) -> date:
        """获取某日的下一个交易日

        Args:
            d (Union[date, datetime]): 某时间

        Returns:
            date: 前一个交易日
        """
        data = await fetch_prev_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    async def fetch_is_trade_date(d: Union[date, datetime]) -> bool:
        """测试某日是否交易日

        Args:
            d (Union[date, datetime]): 某日

        Returns:
            bool: True 交易日 False 非交易日
        """
        return await fetch_is_trade_date(d)

    # bond
    async def fetch_bond_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """获取可转债基本信息

        Args:
            to_frame (bool, optional): 结果是否转换为`pd.DataFrame`格式。 默认: True。

        Returns:
            Union[List[Dict], pd.DataFrame]: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_bond_info()
                    code   name stock_code stock_name listing_date  is_delist
            0    sz127091   科数转债   sz002335       科华数据   2023-09-12          0
            ..        ...    ...        ...        ...          ...        ...
            [540 rows x 6 columns]
        """
        return self._to_dataframe(to_frame,
                                  await self.bond_fetch.fetch_bond_info())

    async def fetch_bond_bar(self, *, code: str, name: str,
                             stock_code: str, stock_name: str,
                             freq: Optional[int] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             skip_rt: bool = True,
                             to_frame=True, ) -> Dict:
        """可转债k线数据

        Args:
            code (str): 可转债代码
            name (str): 可转债名称
            stock_code (str): 正股代码
            stock_name (str): 正股名称
            freq (Optional[int], optional): k线频率, 参考 :class:`~pywqfetch.BarFreq` 。默认None，即日线.
            start (Optional[date], optional): 开始时间，默认None，即上市时间。
            end (Optional[date], optional): 结束时间，默认None，即当前时间。
            skip_rt (bool, optional): 是否忽略实时k线. 默认 True.
            to_frame (bool, optional): 结果bar是否转换为pd.DataFrame 默认 True.

        Returns:
            Dict: 返回字段参考下面示例
            
        >>> await fetch.fetch_stock_bar(code='bj836675')
            {'code': 'bj836675',
            'name': '秉扬科技',
            'freq': 101,
            'bars':          code  name trade_date  open  close  high   low  volume       amount  turnover  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
            0    bj836675  秉扬科技 2018-01-16  4.99   4.98  4.99  4.98      20      9970.00   0.00   -30.83        0.000000        0.000000    1.030120 
            ..        ...      ...             ...             ...         ...  
            [738 rows x 14 columns]}
        """
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

    async def fetch_index_bar(self, code: str, name: Optional[str] = None,
                              freq: Optional[int] = None,
                              start: Optional[date] = None, end: Optional[date] = None, skip_rt: bool = True,
                              to_frame=True) -> Union[Dict, pd.DataFrame]:
        data = await self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                      freq=freq, start=start, end=end,
                                                      skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    async def fetch_stock_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_info())

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

    async def fetch_stock_index(self, *, index_date=None, to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
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

    async def fetch_stock_industry_daily(self, code: str, name: Optional[str] = None,
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

    async def fetch_stock_rt_quot(self, *, code: List[str], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_rt_quot(code))


class BlockFetch:
    """阻塞版本 :class:`~pywqfetch.fetch.Fetch` ，功能一模一样。

    参考: 
    :class:`~pywqfetch.fetch.Fetch`
    """

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
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_trade_date`"""
        data = block_fetch_trade_date()
        if to_frame:
            data = pd.DataFrame(data)
        data.columns = ['trade_date']
        return data

    @staticmethod
    def fetch_next_trade_date(d: Union[date, datetime]) -> date:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_next_trade_date`"""
        data = block_fetch_next_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    def fetch_prev_trade_date(d: Union[date, datetime]) -> date:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_prev_trade_date`"""
        data = block_fetch_prev_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    def fetch_is_trade_date(d) -> bool:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_is_trade_date`"""
        return block_fetch_is_trade_date(d)

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

    def fetch_stock_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_info())

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

    def fetch_stock_rt_quot(self, *, code: List[str], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_rt_quot(code))
