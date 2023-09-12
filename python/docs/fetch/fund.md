# ETF基金数据

### ETF基金基本信息

​`async def fetch_fund_info(self, *, to_frame=True) -> Union[List[Dict], pd.DataFrame]`​​

输入：

|名称|类型|描述|
| ----------| ------| --------------|
|to_frame|bool|是否转`DataFrame`​格式|

输出(仅描述`DataFrame`​)：

|名称|类型|描述|
| ------| ------| ---------------|
|code|str|ETF基金代码<br />|
|name|str|ETF基金名称|

示例:

```python
>>> import pywqfetch as fetch
>>> await fetch.fetch_fund_info()
     code                name
0    sh016002         博时保证金货币ETFC
..        ...    ...    
[836 rows x 2 columns]
```

### ETF基金净值数据

```python
async def fetch_fund_net(self, *, code: str, name: Optional[str] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             to_frame=True) -> Union[List[Dict], pd.DataFrame]:
```

输入：

|名称|类型|描述|
| ----------| -------------| --------------------|
|code|str|ETF基金代码<br />|
|name|str|ETF基金名称|
|freq|int/BarFreq|k线周期|
|start|date|开始时间，默认全部|
|end|date|结束时间，默认全部|
|to_frame|bool|是否转`DataFrame`​​格式|

输出：

|名称|类型|描述|
| ---------------| ----------| --------------------------------|
|code|str|代码|
|name|str|名称|
|trade_date|datetime|交易日|
|net|float|净值<br />|
|net_acc|float|累计净值|
|chg_pct|float|涨跌%|
|apply_status|str|申购状态：场内买入表示正常交易|
|redeem_status|str|赎回状态：场内卖出表示正常交易|

示例:

```python
import pywqfetch as fetch
await fetch.fetch_fund_net(code='sh588370')
     code name trade_date     net  net_acc  chg_pct apply_status  redeem_status
0    sh588370      2023-09-11  0.9001   0.9001     0.74         场内买入   场内卖出
..        ...  ...        ...     ...      ...      ...          ...   
[189 rows x 8 columns]
```

### ETF基金k线数据

```python
async def fetch_fund_bar(self, *, code: str, name: Optional[str] = None,
                             freq: Optional[int] = None,
                             start: Optional[date] = None, end: Optional[date] = None,
                             skip_rt: bool = True,
                             to_frame=True) -> Dict:
```

输入：

|名称|类型|描述|
| ----------| -------------| ------------------------|
|code|str|ETF基金代码<br />|
|name|str|ETF基金名称|
|freq|int/BarFreq|k线周期|
|start|date|开始时间，默认全部|
|end|date|结束时间，默认全部|
|skip_rt|bool|是否忽略实时行情的数据|
|to_frame|bool|是否转`DataFrame`​​格式|

输出：

|名称|类型|描述|
| ------------------------------------------| ----------------------| ----------------------|
|code|str|ETF基金代码<br />|
|name|str|ETF基金名称|
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
In [25]: await fetch.fetch_fund_bar(code='sh588370')
Out[25]: 
{'code': 'sh588370',
 'name': '',
 'freq': 101,
 'bars':          code         name trade_date   open  close   high    low   volume  amount    turnover  chg_pct  volume_chg_pct  amount_chg_pct  hfq_factor 
 0    sh588370  科创50增强策略ETF 2022-12-13  0.995  0.985  0.998  0.985  2885120   286355757.0  108.559998     0.00        0.000000        0.000000 1.0
 ..        ...          ...        ...    ...    ...    ...    ...      ...   

 [184 rows x 14 columns]}
```

‍

‍
