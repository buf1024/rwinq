# 通用数据

### 基础常量

k线周期

```python
class BarFreq:
    Min1 = 1  # 1分钟k线
    Min5 = 5  # 5分钟k线
    Min15 = 15  # 15分钟k线
    Min30 = 30  # 30分钟k线
    Min60 = 60  # 60分钟k线
    Daily = 101  # 日线
    Weekly = 102  # 周线
    Monthly = 103  # 月线
    LooseDaily = 1010  # 日线，在交易日没结束前，显示的是最新值，交易日结束后，同Daily
```

市场品种：

```python
class MarketType:
    Bond = 0  # 可转债
    Fund = 1  # ETF基金
    Stock = 2  # 股票
```

### 交易日数据

​`async def fetch_trade_date(to_frame=True) -> Union[Set[int], pd.DataFrame]`​

输入：

|名称|类型|描述|
| ----------| ------| ------------------|
|to_frame|bool|是否转`DataFrame`​​格式​|

输出(仅描述​`DataFrame`​)：

|名称|类型|描述|
| ------------| ------| --------|
|trade_date|int|交易日|

示例:

```python
In [1]: import pywqfetch as fetch
   ...: await fetch.fetch_trade_date()
Out[1]: 
      trade_date
0       20120104
1       20120105
...          ...
8068    20021231

[8069 rows x 1 columns]
```

### 下一个交易日

​`async def fetch_next_trade_date(d: Union[date, datetime]) -> date`​

输入：

|名称|类型|描述|
| ------| ------| :----: |
|d|-|时间|

输出：

|名称|类型|描述|
| ------| ------| --------|
|-|date|交易日|

示例:

```python
In [3]: from datetime import datetime

In [4]: now = datetime.now()

In [5]: await fetch.fetch_next_trade_date(now)
Out[5]: datetime.date(2023, 9, 13)
```

### 上一个交易日

​`async def fetch_prev_trade_date(d: Union[date, datetime]) -> date`​

输入：

|名称|类型|描述|
| ------| ------| :----: |
|d|-|时间|

输出：

|名称|类型|描述|
| ------| ------| --------|
|-|date|交易日|

示例:

```python
In [7]: await fetch.fetch_prev_trade_date(now)
Out[7]: datetime.date(2023, 9, 11)
```

### 是否交易日

​`async def fetch_is_trade_date(d: Union[date, datetime]) -> bool`​

输入：

|名称|类型|描述|
| ------| ------| :----: |
|d|-|时间|

输出：

|名称|类型|描述|
| ------| ------| ------------|
|-|bool|是否交易日|

示例:

```python
In [12]: await fetch.fetch_is_trade_date(now)
Out[12]: True
```

### 股票实时行情

```python
 async def fetch_rt_quot(self, *, code: List[str], to_frame=True) -> Union[Dict[str, Dict], pd.DataFrame]:
```

注意，该接口是准实时，有几秒的延迟，切勿高频，可以用与股票/可转债/ETF基金，行业概念等等。

输入：

|名称|类型|描述|
| ----------| -----------| --------------|
|code|List[str]|代码列表<br />|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ---------------------------| ----------------------| -----------------------|
|code|str|代码|
|name|str|名称|
|time|datetime|行情时间|
|last_close|float|昨收价|
|open|float|开盘价|
|high|float|最高价|
|low|float|最低价|
|last|float|最后一口价|
|chg|float|涨跌额|
|chg_pct|float|涨跌%|
|volume|int|成交量|
|amount|float|成交额|
|turnover|float|换手率%|
|total_value|float|总市值|
|currency_value|float|流通市值|
|is_trading|bool|是否交易中|

示例:

```python
In [3]: await fetch.fetch_stock_rt_quot(code =['bj873576', 'sz000001'])
Out[3]: 
       code                time  last_close   open  ...  turnover   total_value  currency_value  is_trading
0  sz000001 2023-09-12 15:04:42       11.34  11.35  ...      0.26  2.188988e+11    2.188946e+11       False
1  bj873576 2023-09-12 15:32:21       13.45  13.33  ...      1.01  1.446856e+09    3.959731e+08       False

[2 rows x 15 columns]
```

‍
