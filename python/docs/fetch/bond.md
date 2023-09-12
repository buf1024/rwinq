# 可转债数据

### 可转债基本信息

​`async def fetch_bond_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]`​

输入：

|名称|类型|描述|
| ----------| ------| --------------|
|to_frame|bool|是否转`DataFrame`​格式|

输出(仅描述`DataFrame`​)：

|名称|类型|描述|
| ------------------| ----------| --------------|
|code|str|可转债代码<br />|
|name|str|可转债名称|
|stock\_code|str|正股代码|
|stock\_name|str|正股名称|
|listing\_date|datetime|上市日期|
|is\_delist|int|是否已经退市|

示例:

```python
import pywqfetch as fetch
await fetch.fetch_bond_info()
     code   name stock_code stock_name listing_date  is_delist
0    sz127091   科数转债   sz002335       科华数据   2023-09-12          0
..        ...    ...        ...        ...          ...        ...
[540 rows x 6 columns]
```

### ​​可转债k线数据​

```python
async def fetch_bond_bar(self, *, code: str, name: str,
                             stock_code: str, stock_name: str,
                             freq: Optional[int] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             skip_rt: bool = True,
                             to_frame=True, ) -> Dict:
```

输入：

|名称|类型|描述|
| ----------------| -------------| ------------------------|
|code|str|可转债代码<br />|
|name|str|可转债名称|
|stock\_code|str|正股代码|
|stock\_name|str|正股名称|
|freq|int/BarFreq|k线周期|
|start|date|开始时间，默认全部|
|end|date|结束时间，默认全部|
|skip_rt|bool|是否忽略实时行情的数据|
|to_frame|bool|是否转`DataFrame`​格式|

输出：

|名称|类型|描述|
| ------------------------------------------| ----------------------| ----------------------|
|code|str|可转债代码<br />|
|name|str|可转债名称|
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
In [18]: await fetch.fetch_bond_bar(code='sz128017', name='金禾转债', stock_code='sz002597', stock_name='金禾
    ...: 实业')
Out[18]: 
{'code': 'sz128017',
 'name': '金禾转债',
 'stock_code': 'sz002597',
 'stock_name': '金禾实业',
 'freq': 101,
 'bars':           code  name trade_date        open  ...    chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor
 0     sz128017  金禾转债 2017-11-27  118.660004  ...  16.549999        0.000000        0.000000         1.0
 1     sz128017  金禾转债 2017-11-28  115.970001  ...  -2.240000      -67.826813      -69.116272         1.0
 ...        ...   ...        ...         ...  ...        ...             ...             ...         ...
 1409  sz128017  金禾转债 2023-09-12  117.603996  ...   0.870000      -14.785223      -14.609803         1.0
 
 [1410 rows x 14 columns]}
```
