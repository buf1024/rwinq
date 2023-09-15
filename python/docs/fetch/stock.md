# 股票数据

### 指数基本信息

​`​ async def fetch_index_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:`​

输入：

|名称|类型|描述|
| ----------| ------| --------------|
|to_frame|bool|是否转`DataFrame`​格式|

输出(仅描述`DataFrame`​)：

|名称|类型|描述|
| --------------| ----------| --------------------------|
|code|str|指数代码<br />|
|name|str|指数名称|
|block|str|板块|
|is_margin|bool|是否融资融券，字段无意义|
|listing_date|datetime|上市日期，字段无意义|

示例:

```python
>>> import pywqfetch as fetch
>>> await fetch.fetch_index_info()
      code   name block  is_margin listing_date
0     sh000001   上证综指    指数      False   1970-01-01
..        ...    ...    
[12 rows x 5 columns]
```

### 指数k线数据

```python
async def fetch_index_bar(self, *, code: str, name: Optional[str] = None,
                              freq: Optional[int] = None,
                              start: Optional[date] = None, end: Optional[date] = None, skip_rt: bool = True, to_frame=True) -> Dict:
```

输入：

|名称|类型|描述|
| ----------| -------------| ------------------------|
|code|str|指数代码<br />|
|name|str|指数名称|
|freq|int/BarFreq|k线周期|
|start|date|开始时间，默认全部|
|end|date|结束时间，默认全部|
|skip_rt|bool|是否忽略实时行情的数据|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------------------------------------------| ----------------------| ----------------------|
|code|str|指数代码<br />|
|name|str|指数名称|
|freq|int/BarFreq|k线周期|
|bars|DataFrame|-|
|DataFrame格式|||
|code|str|代码|
|name|str|名称|
|trade_date|datetime|交易日|
|open|float|开盘价|
|close|float|收盘价|
|high|float|最高价|
|low|float|最低价|
|volume|int|成交量|
|amount|float|成交额|
|turnover|float|换手率%|
|chg_pct|float|涨跌%|
|volume_chg_pct|float|成交量变化%|
|amount_chg_pct|float|成交额变化%|
|hfq_factor|float|后复权因子|

示例:

```python
In [3]: await fetch.fetch_index_bar(code='bj899050')
Out[3]: 
{'code': 'bj899050',
 'name': '北证50',
 'freq': 101,
 'bars':          code  name trade_date         open  ...  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
 0    bj899050  北证50 2022-04-29  1000.000000  ...     0.00        0.000000        0.000000         1.0
 ..        ...   ...        ...          ...  ...      ...             ...             ...         ...
 [336 rows x 14 columns]}
```

### 股票基本信息

​`​ async def fetch_stock_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:`​

输入：

|名称|类型|描述|
| ----------| ------| --------------|
|to_frame|bool|是否转`DataFrame`​格式|

输出(仅描述`DataFrame`​)：

|名称|类型|描述|
| --------------| ----------| --------------|
|code|str|指数代码<br />|
|name|str|指数名称|
|block|str|板块|
|is_margin|bool|是否融资融券|
|listing_date|datetime|上市日期|

示例:

```python
>>> import pywqfetch as fetch
>>> await fetch.fetch_stock_info()
      code   name block  is_margin listing_date
0     sh600000   浦发银行    主板       True   1999-11-10
..        ...    ...  
[5056 rows x 5 columns]
```

### 融资股票信息

​`​ async def fetch_stock_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:`​​

输入：

|名称|类型|描述|
| ----------| ------| --------------|
|to_frame|bool|是否转`DataFrame`​格式|

输出(仅描述`DataFrame`​)：

|名称|类型|描述|
| ------| ------| ------------|
|code|str|股票代码<br />|

示例:

```python
>>> import pywqfetch as fetch
>>> fetch.fetch_stock_is_margin()
      code
0     sz003040
...
[3500 rows x 1 columns]
```

### 股票k线数据

```python
async def fetch_stock_bar(self, *, code: str, name: Optional[str] = None,
                              freq: Optional[int] = None,
                              start: Optional[date] = None, end: Optional[date] = None,
                              skip_rt: bool = True,
                              to_frame=True) -> Union[Dict, pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ----------| -------------| ------------------------|
|code|str|股票代码<br />|
|name|str|股票名称|
|freq|int/BarFreq|k线周期|
|start|date|开始时间，默认全部|
|end|date|结束时间，默认全部|
|skip_rt|bool|是否忽略实时行情的数据|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------------------------------------------| ----------------------| ----------------------|
|code|str|股票代码<br />|
|name|str|股票名称|
|freq|int/BarFreq|k线周期|
|bars|DataFrame|-|
|DataFrame格式|||
|code|str|代码|
|name|str|名称|
|trade_date|datetime|交易日|
|open|float|开盘价|
|close|float|收盘价|
|high|float|最高价|
|low|float|最低价|
|volume|int|成交量|
|amount|float|成交额|
|turnover|float|换手率%|
|chg_pct|float|涨跌%|
|volume_chg_pct|float|成交量变化%|
|amount_chg_pct|float|成交额变化%|
|hfq_factor|float|后复权因子|

示例:

```python
In [4]: await fetch.fetch_stock_bar(code='sz003040')
Out[4]: 
{'code': 'sz003040',
 'name': '楚天龙',
 'freq': 101,
 'bars':          code name trade_date   open  ...    chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
 0    sz003040  楚天龙 2021-03-22   5.54  ...  43.939999        0.000000        0.000000    1.000000
 ..        ...  ...        ...    ...  ...        ...             ...             ...         ...
 
 [605 rows x 14 columns]}
```

### ​​股票指标数据​

```python
async def fetch_stock_index(self, *, index_date: Optional[date] = None, to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ------------| ----------| --------------|
|index_date|datetime|指标日期<br />|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------------------------------------------| ----------------------| ----------------------|
|code|str|股票代码<br />|
|name|str|股票名称|
|freq|int/BarFreq|k线周期|
|bars|DataFrame|-|
|DataFrame格式|||
|code|str|代码|
|name|str|名称|
|trade_date|datetime|交易日|
|price|float|当前价|
|pe|float|收盘价|
|pb|float|最高价|
|total_value|float|总市值（元）|
|currency_value|float|流通市值（元）|

示例:

```python
In [5]: await fetch.fetch_stock_index()
Out[5]: 
          code  name trade_date      price          pe     pb   total_value  currency_value
0     sh601992  金隅集团 2023-09-12   2.180000   28.379999   0.50  2.327754e+10    1.816847e+10
...        ...   ...        ...        ...         ...    ...           ...             ...

[5298 rows x 8 columns]
```

### 股票行业数据

```python
async def fetch_stock_industry(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ----------| ------| --------------|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------| ------| ----------|
|code|str|行业代码|
|name|str|行业名称|

示例:

```python
import pywqfetch as fetch
await fetch.fetch_stock_industry()
    code   name
0   BK1015   能源金属
```

### 股票行业明细

```python
async def fetch_stock_industry_detail(self, *, code: Optional[str] = None,
                                          name: Optional[str] = None,
                                          to_frame=True) -> Union[List[Dict], pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ---------------------| --------------------| -------------------------|
|code|str|行业代码|
|name|str|行业名称|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------------| ------| ------------|
|code|str|行业代码<br />|
|name|str|行业名称|
|stock_code|str|股票代码|
|sotck_name|str|股票名称|

示例:

```python
In [9]: await fetch.fetch_stock_industry_detail(code='BK1046', name='游戏')
Out[9]: 
      code name stock_code stock_name
0   BK1046   游戏   sz300052        中青宝
1   BK1046   游戏   sh600715       文投控股
...

In [10]: 
```

### 股票行业日线数据

```python
async def fetch_stock_industry_daily(self, *, code: str, name: Optional[str] = None,
                                         start: Optional[date] = None, end: Optional[date] = None,
                                         skip_rt: bool = True,
                                         to_frame=True) -> Union[Dict, pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ----------| ------| ------------------------|
|code|str|股票代码<br />|
|name|str|股票名称|
|start|date|开始时间，默认全部|
|end|date|结束时间，默认全部|
|skip_rt|bool|是否忽略实时行情的数据|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------------------------------------------| ----------------------| ----------------------|
|code|str|股票代码<br />|
|name|str|股票名称|
|freq|int/BarFreq|k线周期|
|bars|DataFrame|-|
|DataFrame格式|||
|code|str|代码|
|name|str|名称|
|trade_date|datetime|交易日|
|open|float|开盘价|
|close|float|收盘价|
|high|float|最高价|
|low|float|最低价|
|volume|int|成交量|
|amount|float|成交额|
|turnover|float|换手率%|
|chg_pct|float|涨跌%|
|volume_chg_pct|float|成交量变化%|
|amount_chg_pct|float|成交额变化%|
|hfq_factor|float|后复权因子|

示例:

```python
In [10]: await fetch.fetch_stock_industry_daily(code='BK0474')
Out[10]: 
{'code': 'BK0474',
 'name': '保险',
 'freq': 101,
 'bars':         code name trade_date         open  ...  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
 0     BK0474   保险 2007-01-09  1000.000000  ...     5.22        0.000000        0.000000         1.0
 ...      ...  ...        ...          ...  ...      ...             ...             ...         ...

 [4057 rows x 14 columns]}
```

### 股票概念数据

```python
async def fetch_stock_concept(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ----------| ------| --------------|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------| ------| ------------|
|code|str|概念代码<br />|
|name|str|概念名称|

示例:

```python
In [11]: await fetch.fetch_stock_concept()
Out[11]: 
       code   name
0    BK1092   麒麟电池
.. .. ..
[432 rows x 2 columns]
```

### 股票概念明细

```python
async def fetch_stock_concept_detail(self, *, code: Optional[str] = None,
                                          name: Optional[str] = None,
                                          to_frame=True) -> Union[List[Dict], pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ---------------------| --------------------| -------------------------|
|code|str|概念代码|
|name|str|概念名称|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------------| ------| ------------|
|code|str|行业代码<br />|
|name|str|行业名称|
|stock_code|str|股票代码|
|sotck_name|str|股票名称|

示例:

```python
In [13]: await fetch.fetch_stock_concept_detail(code='BK1084', name='数字哨兵')
Out[13]: 
      code  name stock_code stock_name
0   BK1084  数字哨兵   sz300188       美亚柏科
1   BK1084  数字哨兵   sz002908       德生科技
.. .. ..
```

### 股票概念日线数据

```python
async def fetch_stock_concept_daily(self, *, code: str, name: Optional[str] = None,
                                         start: Optional[date] = None, end: Optional[date] = None,
                                         skip_rt: bool = True,
                                         to_frame=True) -> Union[Dict, pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ----------| ------| ------------------------|
|code|str|概念代码<br />|
|name|str|概念名称|
|start|date|开始时间，默认全部|
|end|date|结束时间，默认全部|
|skip_rt|bool|是否忽略实时行情的数据|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------------------------------------------| ----------------------| ----------------------|
|code|str|概念代码<br />|
|name|str|概念名称|
|freq|int/BarFreq|k线周期|
|bars|DataFrame|-|
|DataFrame格式|||
|code|str|代码|
|name|str|名称|
|trade_date|datetime|交易日|
|open|float|开盘价|
|close|float|收盘价|
|high|float|最高价|
|low|float|最低价|
|volume|int|成交量|
|amount|float|成交额|
|turnover|float|换手率%|
|chg_pct|float|涨跌%|
|volume_chg_pct|float|成交量变化%|
|amount_chg_pct|float|成交额变化%|
|hfq_factor|float|后复权因子|

示例:

```python
In [15]: await fetch.fetch_stock_concept_daily(code='BK1084')
Out[15]: 
{'code': 'BK1084',
 'name': '数字哨兵',
 'freq': 101,
 'bars':        code  name trade_date         open  ...  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
 0    BK1084  数字哨兵 2022-05-17   999.750000  ...    -1.16        0.000000        0.000000         1.0
 ..      ...   ...        ...          ...  ...      ...             ...             ...         ...
 
 [327 rows x 14 columns]}
```

### 股票融资融券

```python
 async def fetch_stock_margin(self, *, code: str, start: Optional[date] = None, end: Optional[date] = None,
                                 to_frame=True) -> Union[List[Dict], pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ----------| ------| --------------------|
|code|str|股票代码<br />|
|start|date|开始时间，默认全部|
|end|date|结束时间，默认全部|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| -----------------| ----------| ------------------------------|
|code|str|股票代码<br />|
|name|str|股票名称|
|trade_date|datetime|交易日|
|close|float|收盘价|
|chg_pct|float|涨跌%<br />|
|rz_ye|float|融资: 余额(元)|
|rz_ye_zb|flaot|余额占流通市值比(%)|
|​rz_mre​|float|买入额(元)<br />|
|​rz_che​|float|​偿还额(元)​|
|​rz_jme​|float|​​​净买入(元)​​|
|​rq_ye​|float|​​​​融券: 余额(元)​​​|
|​rq_yl​|float|​余量(股)​|
|​rq_mcl​|float|​卖出量(股)​|
|​rq_chl​|int|​偿还量(股)​|
|​rq_jmg​|int|​净卖出(股)​|
|​rz_rq_ye​|falot|​融资融券余额(元)​|
|​rz_rq_ye_cz​|float|​融资融券余额差值(元)​|

示例:

```python
n [17]: await fetch.fetch_stock_margin(code='sz000001')
Out[17]: 
          code  name trade_date      close  chg_pct  ...  rq_mcl   rq_chl  rq_jmg      rz_rq_ye   rz_rq_ye_cz
0     sz000001  平安银行 2023-09-11  11.340000   0.6211  ...   72300   232700 -160400  4.306517e+09  4.274436e+09
...        ...   ...        ...        ...      ...  ...     ...      ...     ...           ...           ...

[3256 rows x 16 columns]
```

### 业绩报表

```python
async def fetch_stock_yjbb(self, *, year: int, season: int,
                               to_frame=True) -> Union[List[Dict], pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ----------| ------| ------------------|
|year|int|年份|
|season|int|季度: 1, 2, 3, 4|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ---------------------| --------------------| -------------------------|
|year|ing|年份<br />|
|season|ing|季度: 1, 2, 3, 4|
|season_date|datetime|季度时间|
|code|str|股票代码|
|name|str|股票名称|
|mg_sy|float|​​​每股收益​​|
|yysr|float|营业收入<br />|
|yysr_tbzz|float|营业收入-同比增长|
|yysr_jdhbzz|flaot|营业收入-季度环比增长|
|jlr|float|净利润<br />|
|jlr_tbzz|float|净利润-同比增长|
|jlr_jdhbzz|float|净利润-季度环比增长|
|mg_jzc|float|每股净资产|
|jzc_syl|float|净资产收益率|
|mg_jy_xjl|float|每股经营现金流量|
|xs_mll|float|销售毛利率|

示例:

```python
In [20]: await fetch.fetch_stock_yjbb(year=2023, season=2)
Out[20]: 
       year  season season_date      code  name  ...  jlr_jdhbzz    mg_jzc   jzc_syl  mg_jy_xjl     xs_mll
0      2023       2  2023-06-30  sh688657  浩辰软件  ...    0.000000  9.646614  9.646614   0.756221  91.050240
...     ...     ...         ...       ...   ...  ...         ...       ...       ...        ...        ...

[11657 rows x 16 columns]
```

### 股票实时排名

```python
async def fetch_stock_hot_rank(self, *, code: Union[str, list],
                                   to_frame=True) -> Union[Dict, pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ---------------------| -----------------------------------| -----------------------------------|
|code|Union[str, list]<br />|单个或一组股票代码|

输出：

|名称|类型|描述|
| ------------------| ----------| ------------|
|code|str|行业代码<br />|
|market_all_count|str|行业名称|
|rank|int|当前排名|
|rank_chg|int|排名变化|
|calc_time|datetime|当天时间|

示例:

```python
In [21]: await fetch.fetch_stock_hot_rank(code ='sz000001')
Out[21]: 
       code  market_all_count  rank  rank_chg           calc_time
0  sz000001 
```

### 筹码成本分布

```python
def calc_chip_dist(*, data: pd.DataFrame, ac: int = 1, chip_dist: Dict = None) -> Dict:
```

计算筹码分布，采用的是三角分布的算法，不要传递上市以来的所有数据，否则计算比较慢。传递120天或者90天，计算的结果和全量数据结果差不多。

输入：

|名称|类型|描述|
| ---------------------| -------------------------------| ---------------------------------------------------|
|data|pd.DataFrame<br />|股票日线数据, 参考日线数据bars部分|

输出：

|名称|类型|描述|
| -------------------------------------------| ----------------------| --------------------------------|
|chip|Dict|所有价格的筹码分布|
|chip_list|Dict|每日的筹码分布|
|筹码分布Dict<br />|||
|价格|持仓量|key为价格单位分，value为持仓量|

示例:

```python
In [8]: from pywqfetch import *
In [9]: data = await fetch_stock_bar(code='sz000039')
In [10]: data = data['bars']
In [11]: data = data[-10:-1]
In [12]: dist = calc_chip_dist(data=data)
In [13]: dist
Out[13]: 
{'chip': {690: 13.788155302733578,
  ...
  722: 26.93899356690681},
 'chip_list': {20230901: {...},
  ...
  20230913: {...}}}

In [14]: 
```

### 筹码​​获利盘​

```python
def calc_winner(*, chip_dist: Dict, data: pd.DataFrame = None, price: float = None) -> Dict:
```

输入：

|名称|类型|描述|
| --------------------------| -----------------------| ----------------------------------------------------------------------------------------------------|
|chip_dist|Dict|calc_chip_dist的返回值|
|data|Dict<br />|股票日线数据, 参考日线数据bars部分，主要是参考收盘价，data和price两者必须一个不为空|
|price|float|相对该价的盈利，data和price两者必须一个不为空|

输出：

|名称|类型|描述|
| ---------------------------------------| ----------------------| ---------------------------|
|-|Dict|每日的盈利概率|
|盈利概率<br />|||
|日期|Float|key为日期int，value为概率|

示例:

```python
In [14]: calc_winner(chip_dist=dist, data=data)
Out[14]: 
{20230901: 0.6644586478617531,
 20230904: 0.6291703778163454,
 20230905: 0.7535948207068112,
 20230906: 0.2455806272170779,
 20230907: 0.04154799274363736,
 20230908: 0.0,
 20230911: 0.5919655179648782,
 20230912: 0.18903289615357532,
 20230913: 0.5132108034149797}

In [15]: 
```

### 筹码成本分布

```python
def calc_cost(*, chip_dist: Dict, ratio: int) -> Dict
```

输入：

|名称|类型|描述|
| --------------------------| ---------------------| ----------------------------------------------|
|chip_dist|Dict|calc_chip_dist的返回值|
|ratio|int|成本区间，如90，代表90%的筹码|

输出：

|名称|类型|描述|
| ---------------------------------------| ----------------------| ---------------------------|
|-|Dict|每日的盈利概率|
|盈利概率<br />|||
|日期|Float|key为日期int，value为价格|

示例:

```python
In [15]: calc_cost(chip_dist=dist, ratio=90)
Out[15]: 
{20230901: 7.15,
 20230904: 7.2,
 20230905: 7.2,
 20230906: 7.19,
 20230907: 7.19,
 20230908: 7.18,
 20230911: 7.18,
 20230912: 7.18,
 20230913: 7.17}
```

‍
