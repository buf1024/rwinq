from typing import Tuple
from hiq_pystrategy.strategy import Strategy


class BaseStrategy(Strategy):
    def __init__(self, loader, fetch, cmm_params, params) -> None:
        super().__init__(loader, fetch, cmm_params, params)

    @staticmethod
    def shadow(open: float, close: float, low: float, high: float) -> Tuple[float, float, float]:
        if high == low:
            return (0, 0)
        base = high - low
        is_up = close > open
        if is_up:
            return ((high - close)*100 / base,
                    (close - open)*100 / base,
                    (open - low)*100 / base
                    )
        return ((high - open)*100 / base,
                (open - close)*100 / base,
                (close - low)*100 / base
                )

        # for s in side:
        #     if s == 'top':
        #         r = (high - close) * 100 / (high - low)
        #         if r > ratio:
        #             return True
        #     if s == 'bottom':
        #         r = (open_ - low) * 100 / (high - low)
        #         if r > ratio:
        #             return True

        # return False
