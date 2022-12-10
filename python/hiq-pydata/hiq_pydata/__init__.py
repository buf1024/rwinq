from typing import Optional
from hiq_pydata.hiq_loader import HiqLoader
from hiq_pydata.hiq_mongo import HiqMongoLoader, HiqBlockMongoLoader

from hiq_pydata.hiq_sync import MyHiqSync, MyBlockHiqSync, Dest, Funcs


def get_loader(typ: str, url: str, block: bool = False) -> Optional[HiqLoader]:
    typ = typ.lower()
    loader = None
    if typ == "file":
        pass
    elif typ == "mongodb":
        loader = HiqMongoLoader(
            url=url) if not block else HiqBlockMongoLoader(url=url)
    elif typ == "mysql":
        pass
    elif typ == "clickhouse":
        pass

    return loader
