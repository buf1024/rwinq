"""原始Rust包装类的汇聚类，包括可转债，ETF基金以及股票三大类，原始类有的功能此类保留，同时增加额外功能，通常使用该汇集类来获取数据。

请求返回的数据比较统一，通常为原始的Rust代码值或者Rust返回的Dict数据转换为pandas.Dataframe格式。

比如bar的pandas.Dataframe格式返回：

>>> {code: 'bj836675',
    'name': '秉扬科技',
    'freq': 101,
    'bars':          code  name trade_date  open  close  high   low  volume       amount  turnover  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
    ...
    }

"""

from datetime import date, datetime
from typing import List, Dict, Union, Optional, Set

from pywqfetch.pywqfetch import BlockBondFetch, BlockFundFetch, BlockStockFetch, BondFetch, FundFetch, StockFetch, block_fetch_next_trade_date, block_fetch_prev_trade_date, fetch_next_trade_date, fetch_is_trade_date, block_fetch_is_trade_date, fetch_prev_trade_date, fetch_trade_date, block_fetch_trade_date

import pandas as pd


class Fetch:
    """
    原始Rust包装类的汇聚类，包括可转债，ETF基金以及股票三大类。

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
        """获取交易日历

        Args:
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame格式。 默认: True。

        Returns:
            Union[Set[int], pd.DataFrame]: to_frame 为 False时，返回Set， 否则返回pandas.DataFrame。

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
        """获取某个时间的下一个交易日

        Args:
            d (Union[date, datetime]): 某个时间

        Returns:
            date: 下一个交易日
        """
        data = await fetch_next_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    async def fetch_prev_trade_date(d: Union[date, datetime]) -> date:
        """获取某个时间的下一个交易日

        Args:
            d (Union[date, datetime]): 某个时间

        Returns:
            date: 前一个交易日
        """
        data = await fetch_prev_trade_date(d)
        return datetime.strptime('{} 00:00:00'.format(data), '%Y%m%d %H:%M:%S').date()

    @staticmethod
    async def fetch_is_trade_date(d: Union[date, datetime]) -> bool:
        """测试某个时间是否交易日

        Args:
            d (Union[date, datetime]): 某个时间

        Returns:
            bool: True 交易日 False 非交易日
        """
        return await fetch_is_trade_date(d)

    # bond
    async def fetch_bond_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """获取可转债基本信息

        Args:
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame格式。 默认: True。

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
            to_frame (bool, optional): 结果bar是否转换为pandas.DataFrame 默认 True.

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
        """获取ETF基金基本信息

        Args:
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame格式。 默认: True。

        Returns:
            Union[List[Dict], pd.DataFrame]: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_bond_info()
                     code                name
            0    sh016002         博时保证金货币ETFC
            ..        ...    ...        
            [836 rows x 2 columns]
        """
        return self._to_dataframe(to_frame,
                                  await self.fund_fetch.fetch_fund_info())

    async def fetch_fund_net(self, *, code: str, name: Optional[str] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """ETF基金净值数据

        Args:
            code (str): ETF基金代码
            name (str): ETF基金名称
            start (Optional[date], optional): 开始时间，默认None，即上市时间。
            end (Optional[date], optional): 结束时间，默认None，即当前时间。
            skip_rt (bool, optional): 是否忽略实时k线. 默认 True.
            to_frame (bool, optional): 结果bar是否转换为pandas.DataFrame 默认 True.

        Returns:
            Union[List[Dict], pd.DataFrame]: 返回字段参考下面示例
            net 净值 net_acc 累计净值 apply_status 申购状态 redeem_status 赎回状态

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_fund_net(code='sh588370')
                     code name trade_date     net  net_acc  chg_pct apply_status  redeem_status
            0    sh588370      2023-09-11  0.9001   0.9001     0.74         场内买入   场内卖出
            ..        ...  ...        ...     ...      ...      ...          ...   
            [189 rows x 8 columns]
        """
        return self._to_dataframe(to_frame,
                                  await self.fund_fetch.fetch_fund_net(code=code, name=name,
                                                                       start=start, end=end))

    async def fetch_fund_bar(self, *, code: str, name: Optional[str] = None,
                             freq: Optional[int] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             skip_rt: bool = True,
                             to_frame=True) -> Dict:
        """ETF基金k线数据

        Args:
            code (str): ETF基金代码
            name (str): ETF基金名称
            freq (Optional[int], optional): k线频率, 参考 :class:`~pywqfetch.BarFreq` 。默认None，即日线.
            start (Optional[date], optional): 开始时间，默认None，即上市时间。
            end (Optional[date], optional): 结束时间，默认None，即当前时间。
            skip_rt (bool, optional): 是否忽略实时k线. 默认 True.
            to_frame (bool, optional): 结果bar是否转换为pandas.DataFrame 默认 True.

        Returns:
            Dict: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_fund_bar(code='bj836675')
            {'code': 'sh588370',
            'name': '',
            'freq': 101,
            'bars':          code  name trade_date  open  close  high   low  volume       amount  turnover  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
            0    sh588370  科创50增强策略ETF 2022-12-13  0.995  0.985  0.998  0.985  2885120 286355757.0  107.540001     0.00        0.000000        0.000000 1.0
            ..        ...      ...             ...             ...         ...  
            [183 rows x 14 columns]}
        """
        data = await self.fund_fetch.fetch_fund_bar(code=code, name=name,
                                                    freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    # stock
    async def fetch_index_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """获取股票指数基本信息

        Args:
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame格式。 默认: True。

        Returns:
            Union[List[Dict], pd.DataFrame]: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_index_info()
                     code   name block  is_margin listing_date
            0     sh000001   上证综指    指数      False   1970-01-01
            ..        ...    ...        
            [12 rows x 4 columns]
        """
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_index_info())

    async def fetch_index_bar(self, *, code: str, name: Optional[str] = None,
                              freq: Optional[int] = None,
                              start: Optional[date] = None, end: Optional[date] = None, skip_rt: bool = True,
                              to_frame=True) -> Dict:
        """指数k线数据

        Args:
            code (str): 指数代码
            name (str): 指数名称
            freq (Optional[int], optional): k线频率, 参考 :class:`~pywqfetch.BarFreq` 。默认None，即日线.
            start (Optional[date], optional): 开始时间，默认None，即上市时间。
            end (Optional[date], optional): 结束时间，默认None，即当前时间。
            skip_rt (bool, optional): 是否忽略实时k线. 默认 True.
            to_frame (bool, optional): 结果bar是否转换为pandas.DataFrame 默认 True.

        Returns:
            Dict: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_index_bar(code='bj899050')
            {'code': 'bj899050',
            'name': '北证50',
            'freq': 101,
            'bars':          code  name trade_date  open  close  high   low  volume       amount  turnover  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
            0    bj899050  北证50 2022-04-29  1000.000000  1000.000000  1000.000000 1000.000000  45521550  5.819064e+08  29.370001     0.00        0.000000 0.000000         1.0 
            ..        ...      ...             ...             ...         ...  
            [183 rows x 14 columns]}
        """
        data = await self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                      freq=freq, start=start, end=end,
                                                      skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    async def fetch_stock_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """获取股票基本信息

        Args:
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame格式。 默认: True。

        Returns:
            Union[List[Dict], pd.DataFrame]: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_stock_info()
                    code   name block  is_margin listing_date
            0     sh600000   浦发银行    主板       True   1999-11-10
            ..        ...    ...        ...        ...          ...        ...
            [5056 rows x 5 columns]
        """
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_info())

    async def fetch_stock_is_margin(self, *, to_frame=True) -> Union[Set[str], pd.DataFrame]:
        """获取融资融券股票

        Args:
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame格式。 默认: True。

        Returns:
            Union[Set[str], pd.DataFrame]: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_stock_is_margin()
                    code
            0     sz003040
            ...
            [3500 rows x 1 columns]
        """
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
        """股票k线数据

        Args:
            code (str): 股票代码
            name (str): 股票名称
            freq (Optional[int], optional): k线频率, 参考 :class:`~pywqfetch.BarFreq` 。默认None，即日线.
            start (Optional[date], optional): 开始时间，默认None，即上市时间。
            end (Optional[date], optional): 结束时间，默认None，即当前时间。
            skip_rt (bool, optional): 是否忽略实时k线. 默认 True.
            to_frame (bool, optional): 结果bar是否转换为pandas.DataFrame 默认 True.

        Returns:
            Dict: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_stock_bar(code='sz003040')
            {'code': 'sz003040',
            'name': '楚天龙',
            'freq': 101,
            'bars':          code  name trade_date  open  close  high   low  volume       amount  turnover  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
            0    sz003040  楚天龙 2021-03-22   5.54   6.650000   6.650000   5.540000    5355 3545241.00      0.68  43.939999        0.000000        0.000000         1.0 
            ..        ...      ...             ...             ...         ...  
            [604 rows x 14 columns]}
        """
        data = await self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                      freq=freq, start=start, end=end,
                                                      skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    async def fetch_stock_index(self, *, index_date: Optional[date] = None, to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        """获取股票指标数据

        Args:
            index_date (Optional[date], optional): 指标日期. 默认索引交易日.
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame 默认 True.

        Returns:
            Union[Dict[str, Dict], pd.DataFrame]: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_stock_index(code='sz003040')
                  code   name trade_date  price          pe    pb   total_value  currency_value
            0     sz300322    硕贝德 2023-09-12   8.69  -15.470000  3.67  4.047336e+09 3.854398e+09 
            ...
            [5298 rows x 8 columns]
        """
        data = await self.stock_fetch.fetch_stock_index(index_date)
        if to_frame:
            data = pd.DataFrame(list(data.values()))
        return data

    async def fetch_stock_industry(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """获取股票行业信息

        Args:
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame格式。 默认: True。

        Returns:
            Union[List[Dict], pd.DataFrame]: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_stock_industry()
                    code   name
            0   BK1015   能源金属
            ..        ...
            [86 rows x 2 columns]
        """
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_industry())

    async def fetch_stock_industry_detail(self, *, code: Optional[str] = None,
                                          name: Optional[str] = None,
                                          to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """获取行业内股票信息

        Args:
            code (Optional[str], optional): 行业代码，默认所有行业.
            name (Optional[str], optional): 行业名称，默认空，该字段用于填充结果集.
            to_frame (bool, optional): 结果是否转换为pandas.DataFrame格式。 默认: True。

        Returns:
            Union[List[Dict], pd.DataFrame]: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_stock_industry_detail()
                    code  name stock_code stock_name
            0     BK1015  能源金属   sz002192       融捷股份
            ..        ...
            [5361 rows x 4 columns]
        """
        return self._to_dataframe(to_frame,
                                  await self.stock_fetch.fetch_stock_industry_detail(code, name))

    async def fetch_stock_industry_daily(self, *, code: str, name: Optional[str] = None,
                                         start: Optional[date] = None, end: Optional[date] = None,
                                         skip_rt: bool = True,
                                         to_frame=True) -> Union[Dict, pd.DataFrame]:

        """行业k线数据

        Args:
            code (str): 行业代码
            name (Optional[str]): 行业名称
            freq (Optional[int], optional): k线频率, 参考 :class:`~pywqfetch.BarFreq` 。默认None，即日线.
            start (Optional[date], optional): 开始时间，默认None，即上市时间。
            end (Optional[date], optional): 结束时间，默认None，即当前时间。
            skip_rt (bool, optional): 是否忽略实时k线. 默认 True.
            to_frame (bool, optional): 结果bar是否转换为pandas.DataFrame 默认 True.

        Returns:
            Dict: 返回字段参考下面示例

        >>> import pywqfetch as fetch
        >>> await fetch.fetch_stock_bar(code='sz003040')
            {'code': 'sz003040',
            'name': '楚天龙',
            'freq': 101,
            'bars':          code  name trade_date  open  close  high   low  volume       amount  turnover  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
            0    sz003040  楚天龙 2021-03-22   5.54   6.650000   6.650000   5.540000    5355 3545241.00      0.68  43.939999        0.000000        0.000000         1.0 
            ..        ...      ...             ...             ...         ...  
            [604 rows x 14 columns]}
        """
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
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_bond_info`"""
        return self._to_dataframe(to_frame,
                                  self.bond_fetch.fetch_bond_info())

    def fetch_bond_bar(self, *, code: str, name: str,
                       stock_code: str, stock_name: str,
                       freq: Optional[int] = None,
                       start: Optional[date] = None, end: Optional[date] = None,
                       skip_rt: bool = True,
                       to_frame=True, ) -> Dict:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_bond_bar`"""
        data = self.bond_fetch.fetch_bond_bar(code=code, name=name,
                                              stock_code=stock_code, stock_name=stock_name,
                                              freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    # fund
    def fetch_fund_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_fund_info`"""
        return self._to_dataframe(to_frame,
                                  self.fund_fetch.fetch_fund_info())

    def fetch_fund_net(self, *, code: str, name: Optional[str] = None,
                       start: Optional[date] = None, end: Optional[date] = None,
                       to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_fund_net`"""
        return self._to_dataframe(to_frame,
                                  self.fund_fetch.fetch_fund_net(code=code, name=name,
                                                                 start=start, end=end))

    def fetch_fund_bar(self, *, code: str, name: Optional[str] = None,
                       freq: Optional[int] = None,
                       start: Optional[date] = None, end: Optional[date] = None,
                       skip_rt: bool = True,
                       to_frame=True) -> Dict:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_fund_bar`"""
        data = self.fund_fetch.fetch_fund_bar(code=code, name=name,
                                              freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    # stock
    def fetch_index_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_index_info`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_index_info())

    def fetch_index_bar(self, code: str, name: Optional[str] = None,
                        freq: Optional[int] = None,
                        start: Optional[date] = None, end: Optional[date] = None,
                        skip_rt: bool = True,
                        to_frame=True) -> Union[Dict, pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_index_bar`"""
        data = self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    def fetch_stock_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_info`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_info())

    def fetch_stock_is_margin(self, *, to_frame=True) -> Union[Set[str], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_is_margin`"""
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
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_bar`"""
        data = self.stock_fetch.fetch_stock_bar(code=code, name=name,
                                                freq=freq, start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    def fetch_stock_index(self, *, index_date=None, to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_index`"""
        data = self.stock_fetch.fetch_stock_index(index_date)
        if to_frame:
            data = pd.DataFrame(list(data.values()))
        return data

    def fetch_stock_industry(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_industry`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_industry())

    def fetch_stock_industry_detail(self, *, code: Optional[str] = None,
                                    name: Optional[str] = None,
                                    to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_industry_detail`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_industry_detail(code, name))

    def fetch_stock_industry_daily(self, code: str, name: Optional[str] = None,
                                   start: Optional[date] = None, end: Optional[date] = None,
                                   skip_rt: bool = True,
                                   to_frame=True) -> Union[Dict, pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_industry_daily`"""

        data = self.stock_fetch.fetch_stock_industry_daily(code=code, name=name,
                                                           start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    def fetch_stock_concept(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_concept`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_concept())

    def fetch_stock_concept_detail(self, *, code: Optional[str] = None, name: Optional[str] = None,
                                   to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_concept_detail`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_concept_detail(code, name))

    def fetch_stock_concept_daily(self, *, code: str, name: Optional[str] = None,
                                  start: Optional[date] = None, end: Optional[date] = None,
                                  skip_rt: bool = True,
                                  to_frame=True) -> Union[Dict, pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_concept_daily`"""
        data = self.stock_fetch.fetch_stock_industry_daily(code=code, name=name,
                                                           start=start, end=end, skip_rt=skip_rt)
        data['bars'] = self._to_dataframe(to_frame, data['bars'])
        return data

    def fetch_stock_yjbb(self, *, year: int, season: int,
                         to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_yjbb`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_yjbb(year, season))

    def fetch_stock_margin(self, *, code: str, start: Optional[date] = None, end: Optional[date] = None,
                           to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_margin`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_margin(code, start, end))

    def fetch_stock_hot_rank(self, *, code: Union[str, list],
                             to_frame=True) -> Union[Dict, pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_hot_rank`"""
        codes = code
        if type(code) == type(''):
            codes = [code]
        datas = []
        for code in codes:
            data = self.stock_fetch.fetch_stock_hot_rank(code=code)
            datas.append(data)
        return self._to_dataframe(to_frame, data=datas)

    def fetch_stock_rt_quot(self, *, code: List[str], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
        """阻塞版本, 同 :func:`~pywqfetch.Fetch.fetch_stock_rt_quot`"""
        return self._to_dataframe(to_frame,
                                  self.stock_fetch.fetch_stock_rt_quot(code))
