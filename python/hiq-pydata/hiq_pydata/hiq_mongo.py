from typing import List, Dict, Union, Optional

from hiq_pydata.hiq_pydata import BlockMongoFetch, MongoFetch

import pandas as pd
import json


class MyMongoFetch:
    def __init__(self, url: str):
        self.fetch = MongoFetch(url)

    @staticmethod
    def _to_dataframe(to_frame, data):
        if to_frame and data is not None:
            return pd.DataFrame(data)
        return data

    async def load_bond_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_bond_info(filter=filter, sort=sort, limit=limit))

    async def load_bond_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_bond_daily(filter=filter, sort=sort, limit=limit))

    async def load_fund_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_fund_info(filter=filter, sort=sort, limit=limit))

    async def load_fund_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_fund_daily(filter=filter, sort=sort, limit=limit))

    async def load_fund_net(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_fund_net(filter=filter, sort=sort, limit=limit))

    async def load_index_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_index_info(filter=filter, sort=sort, limit=limit))

    async def load_index_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_index_daily(filter=filter, sort=sort, limit=limit))

    async def load_stock_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_info(filter=filter, sort=sort, limit=limit))

    async def load_stock_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_daily(filter=filter, sort=sort, limit=limit))

    async def load_stock_index(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_index(filter=filter, sort=sort, limit=limit))

    async def load_stock_industry(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_industry(filter=filter, sort=sort, limit=limit))

    async def load_stock_industry_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_industry_daily(filter=filter, sort=sort, limit=limit))

    async def load_stock_industry_detail(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_industry_detail(filter=filter, sort=sort, limit=limit))

    async def load_stock_concept(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_concept(filter=filter, sort=sort, limit=limit))

    async def load_stock_concept_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_concept_daily(filter=filter, sort=sort, limit=limit))

    async def load_stock_concept_detail(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_concept_detail(filter=filter, sort=sort, limit=limit))

    async def load_stock_yjbb(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_yjbb(filter=filter, sort=sort, limit=limit))

    async def load_stock_margin(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  await self.fetch.load_stock_margin(filter=filter, sort=sort, limit=limit))


class MyBlockMongoFetch:
    def __init__(self, url: str):
        self.fetch = BlockMongoFetch(url)

    @staticmethod
    def _to_dataframe(to_frame, data):
        if to_frame and data is not None:
            return pd.DataFrame(data)
        return data

    def load_bond_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_bond_info(filter=filter, sort=sort, limit=limit))

    def load_bond_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_bond_daily(filter=filter, sort=sort, limit=limit))

    def load_fund_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_fund_info(filter=filter, sort=sort, limit=limit))

    def load_fund_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_fund_daily(filter=filter, sort=sort, limit=limit))

    def load_fund_net(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_fund_net(filter=filter, sort=sort, limit=limit))

    def load_index_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_index_info(filter=filter, sort=sort, limit=limit))

    def load_index_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_index_daily(filter=filter, sort=sort, limit=limit))

    def load_stock_info(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_info(filter=filter, sort=sort, limit=limit))

    def load_stock_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_daily(filter=filter, sort=sort, limit=limit))

    def load_stock_index(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_index(filter=filter, sort=sort, limit=limit))

    def load_stock_industry(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_industry(filter=filter, sort=sort, limit=limit))

    def load_stock_industry_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_industry_daily(filter=filter, sort=sort, limit=limit))

    def load_stock_industry_detail(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_industry_detail(filter=filter, sort=sort, limit=limit))

    def load_stock_concept(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_concept(filter=filter, sort=sort, limit=limit))

    def load_stock_concept_daily(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_concept_daily(filter=filter, sort=sort, limit=limit))

    def load_stock_concept_detail(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_concept_detail(filter=filter, sort=sort, limit=limit))

    def load_stock_yjbb(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_yjbb(filter=filter, sort=sort, limit=limit))

    def load_stock_margin(
        self, *,
        filter: Optional[Dict] = {},
        sort: Optional[Dict] = {},
        limit: Optional[int] = None, to_frame=True
    ) -> Union[List[Dict], pd.DataFrame]:

        filter, sort = json.dumps(filter, default=str), json.dumps(sort, default=str)
        return self._to_dataframe(to_frame,
                                  self.fetch.load_stock_margin(filter=filter, sort=sort, limit=limit))
