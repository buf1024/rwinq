from typing import Dict, List, Optional
from hiq_pydata.hiq_loader import HiqLoader
from hiq_pyfetch.hiq_fetch import HiqFetch
from hiq_pystrategy.strategy import CommonParam, StrategyResult, StrategyType
import pandas as pd
from hiq_py.hiq_py.strategy.base_strategy import BaseStrategy


class RightSide(BaseStrategy):
    """
    看上方有没有抛压，上涨幅度。
    貌似上涨30%的都可以追，上涨大于40%有一定下跌的概率
    示意形态:
       |
      |
     |
    |
    """

    def __init__(self, loader: Optional[HiqLoader] = None, fetch: Optional[HiqFetch] = None, cmm_params: Optional[CommonParam] = None, params: Optional[Dict] = None) -> None:
        super().__init__(loader, fetch, cmm_params, params)

        self.min_rise_days = 3
        self.max_shadow_pct = 20
        self.min_vol_chg_pct = -2.0
        self.min_amt_chg_pct = -2.0

    def help(self) -> str:
        return '  名称: 右侧策略(基于日线)\n' + \
               '  说明: 选择右侧温和上涨的标的。\n' + \
               '       \n' + \
               '  参数: min_rise_days -- 最近最小连续上涨天数(默认: 3)\n' + \
               '        max_down_in_rise -- 最大下跌百分比(默认: -2.0)\n' + \
               '        max_up_in_rise -- 最大上涨百分比(默认: 20.0)\n' + \
               '        max_leg_ratio -- 上涨最大腿长(默认: 33.3)\n' + \
               '        recent_days -- 最近累计计算涨幅天数(默认: 8)\n' + \
               '        recent_days_up -- 最近judge_days内上涨百分比(默认: 15.0)'

    async def test(self, typ: StrategyType, code: str, name: str) -> Optional[StrategyResult]:
        self.logger.debug(
            'testing typ: {}, code: {}, name: {}'.format(typ, code, name))
        kdata = await self.load_kdata(typ=typ,
                                      filter={'code': code,
                                              'trade_date': {'$lte': self.test_end_date}},
                                      limit=self.test_trade_days,
                                      sort=[('trade_date', -1)])

        if kdata is None or kdata.shape[0] < self.test_trade_days:
            return None

        fit_days = 0
        for df in kdata.to_dict('records'):
            chg_pct, vol_chg_pct, amt_chg_pct = df['chg_pct'], df['vol_chg_pct'], df['amt_chg_pct']
            open, close, high, low = df['open'], df['close'], df['high'], df['low'],
            _, u_shadow, _, l_shadow = self.shadow(close*(1+chg_pct/100.0),
                                                            open, close, low, high)
            if chg_pct > 0 and \
                    vol_chg_pct >= self.min_vol_chg_pct and \
                    amt_chg_pct >= self.min_amt_chg_pct and \
                    u_shadow <= self.max_shadow_pct and \
                    l_shadow <= self.max_shadow_pct:
                fit_days = fit_days + 1
                continue
            break

        if fit_days < self.min_rise_days:
            return None

        
        re_close, now_close = kdata.iloc[self.recent_ndays -
                                         1]['close'], kdata.iloc[0]['close']
        recent_rise = round((now_close - re_close) * 100 / re_close, 2)
        if recent_rise >= self.recent_days_up:
            name = await self.code_name(code=code, name=name)
            got_data = dict(code=code, name=name,
                            nday_close=re_close, close=now_close, nday_rise=recent_rise,
                            rise_start=kdata.iloc[fit_days]['trade_date'],
                            rise_days=fit_days)
            return pd.DataFrame([got_data])

        return None


if __name__ == '__main__':

    fund, stock, mysql = default(log_level='error')
    s = RightSide(db=stock)

    async def tt():
        await s.prepare(min_rise_days=2, max_down_in_rise=-1,
                        max_up_in_rise=20, max_leg_ratio=40,
                        recent_days=8, recent_days_up=10, sort_by='rise', )
        df = await s.test('sz000558')
        print(df)

    run_until_complete(tt())
