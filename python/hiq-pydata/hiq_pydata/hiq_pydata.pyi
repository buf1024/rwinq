from typing import List, Dict, Optional


class HiqSync:
    def __init__(dest: List[(str, str)], funcs: Optional[List[int]] = None):
        pass

    async def sync(self, skip_basic, task_count, split_count):
        pass

    def shutdown(self):
        pass


class BlockHiqSync:
    def __init__(dest: List[(str, str)], funcs: Optional[List[int]]):
        pass

    def sync(self, skip_basic, task_count, split_count):
        pass

    def shutdown(self):
        pass


class MongoFetch:
    def __init__(url: str):
        pass

    async def load_bond_info(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_bond_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_fund_info(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_fund_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_fund_net(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_index_info(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_index_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_info(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_index(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_industry(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_industry_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_industry_detail(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_concept(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_concept_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_concept_detail(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_yjbb(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    async def load_stock_margin(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass


class BlockMongoFetch:
    def __init__(url: str):
        pass

    def load_bond_info(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass
    
    def load_bond_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_fund_info(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_fund_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_fund_net(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_index_info(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_index_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_info(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_index(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_industry(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_industry_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_industry_detail(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_concept(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_concept_daily(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_concept_detail(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_yjbb(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass

    def load_stock_margin(
        self,
        filter: Optional[str],
        sort: Optional[str],
        limit: Optional[int],
    ) -> List[Dict]:
        pass
