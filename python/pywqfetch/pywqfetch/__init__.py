from pywqfetch.fetch import *


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


class Market:
    SZ = 0
    SH = 1
    BJ = 2


class MarketType:
    Bond = 0  # 可转债
    Fund = 1  # ETF基金
    Stock = 2  # 股票
