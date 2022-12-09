import logging
from typing import Dict, List, Optional, Tuple, Union
from hiq_pydata.hiq_loader import HiqLoader
from hiq_pyfetch.hiq_fetch import HiqFetch
from hiq_pystrategy.strategy import CommonParam, Strategy, StrategyType
import pandas as pd


class BaseStrategy(Strategy):
    def __init__(self, strategy_type: List[int] = [StrategyType.Stock], loader: Optional[HiqLoader] = None, fetch: Optional[HiqFetch] = None, cmm_params: Optional[CommonParam] = None, params: Optional[Dict] = None) -> None:
        super().__init__(loader, fetch, cmm_params, params)
        
        self.strategy_type = strategy_type
        self.logger = logging.getLogger(self.__class__.__name__)

    def accept(self) -> List[int]:
        return self.strategy_type

    @staticmethod
    def shadow(last_close: float, open: float, close: float, low: float, high: float) -> Tuple[float, float, float, float]:
        if high == low:
            return (0, 0)
        amp = (high - low)*100/last_close
        base = high - low
        is_up = close > open
        if is_up:
            return (amp, (high - close)*100 / base,
                    (close - open)*100 / base,
                    (open - low)*100 / base
                    )
        return (amp, (high - open)*100 / base,
                (open - close)*100 / base,
                (close - low)*100 / base
                )

    async def load_kdata(self, typ: StrategyType, filter: Optional[Dict] = {},
                         sort: Optional[Dict] = {},
                         limit: Optional[int] = None, to_frame=True) -> Union[List[Dict], pd.DataFrame]:
        if self.loader is None:
            return None

        funcs = {
            StrategyType.Bond: self.loader.load_bond_daily,
            StrategyType.Fund: self.loader.load_fund_daily,
            StrategyType.Stock: self.loader.load_stock_daily,
            StrategyType.Index: self.loader.load_index_daily,
            StrategyType.Concept: self.loader.load_stock_concept_daily,
            StrategyType.Industry: self.loader.load_stock_industry_daily
        }

        if typ not in funcs.keys():
            return None

        return await funcs[typ](filter=filter, sort=sort, limit=limit, to_frame=to_frame)
