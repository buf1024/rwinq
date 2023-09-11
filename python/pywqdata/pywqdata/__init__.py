from typing import Optional
from pywqdata.loader import Loader
from pywqdata.mongo import MongoLoader, BlockMongoLoader

from pywqdata.sync import MySync, MyBlockSync, Dest, Funcs


def get_loader(typ: str, url: str, block: bool = False) -> Optional[Loader]:
    typ = typ.lower()
    loader = None
    if typ == "file":
        pass
    elif typ == "mongodb":
        loader = MongoLoader(
            url=url) if not block else BlockMongoLoader(url=url)
    elif typ == "mysql":
        pass
    elif typ == "clickhouse":
        pass

    return loader
