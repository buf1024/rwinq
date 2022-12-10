from hiq_pydata import get_loader
from hiq_pyfetch.hiq_fetch import HiqFetch
from hiq_pystrategy.hiq_runner import HiqRunner
import nest_asyncio

nest_asyncio.apply()


async def strategy_basic(data_type: str = 'mongodb', data_url: str = 'mongodb://localhost:27017'):
    loader = get_loader(data_type, data_url)
    runner = HiqRunner('mongodb', 'mongodb://localhost:27017', 50)
    fetch = HiqFetch()

    return loader, runner, fetch
