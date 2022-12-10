from typing import Dict, List, Optional
from hiq_py.common import load_strategy
from os.path import dirname
from hiq_pydata import get_loader
from hiq_pyfetch.hiq_fetch import HiqFetch
from hiq_pystrategy.hiq_runner import HiqRunner

from hiq_pystrategy.strategy import CommonParam, StrategyType


__file_path = dirname(__file__)

strategies = load_strategy(
    __file_path, 'hiq_py.strategy', ('base_strategy.py',))
