from typing import List, Dict, Union, Optional
import pandas as pd
from abc import ABC


def _to_dataframe(to_frame, data):
    if to_frame and data is not None:
        return pd.DataFrame(data)
    return data


def _json_def_handler(obj):
    if hasattr(obj, 'isoformat'):
        return obj.isoformat()
    return None


class HiqLoader(ABC):
    def __init__(self, url: str):
        pass

    @staticmethod
    def to_dataframe(to_frame, data):
        return _to_dataframe(to_frame, data)

    @staticmethod
    def json_def_handler(obj):
        return _json_def_handler(obj)

    async def load_bond_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_bond_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_fund_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_fund_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_fund_net(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_index_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_index_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_index(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_industry(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_industry_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_industry_detail(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_concept(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_concept_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_concept_detail(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_yjbb(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    async def load_stock_margin(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass


class BlockHiqLoader(ABC):
    def __init__(self, url: str):
        pass

    @staticmethod
    def to_dataframe(to_frame, data):
        return _to_dataframe(to_frame, data)

    def load_bond_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_bond_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_fund_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_fund_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_fund_net(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_index_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_index_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_index(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_industry(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_industry_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_industry_detail(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_concept(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_concept_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_concept_detail(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_yjbb(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass

    def load_stock_margin(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:
        pass
