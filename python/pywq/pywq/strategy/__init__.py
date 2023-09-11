from typing import Dict, List, Optional
from rwqpy.common import load_strategy
from os.path import dirname
from pywqdata import get_loader
from rwqpyfetch.rwqfetch import Fetch
from pywqstrategy.rwqrunner import Runner

from pywqstrategy.strategy import CommonParam, StrategyType


__file_path = dirname(__file__)

strategies = load_strategy(
    __file_path, 'rwqpy.strategy', ('base_strategy.py',))
