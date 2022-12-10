import asyncio
import importlib
import logging
import os
from functools import wraps


def singleton(cls):
    insts = {}

    @wraps(cls)
    def wrapper(*args, **kwargs):
        if cls.__qualname__ not in insts:
            insts[cls.__qualname__] = cls(*args, **kwargs)
            cls.inst = insts[cls.__qualname__]
        return insts[cls.__qualname__]

    return wrapper


def load_strategy(dir_path, package, exclude=()):
    """
    strategy文件只能有一个类，类名为文件名(首字母大写), 如文件明带_, 去掉后，后面单词首字母大写
    :param dir_path: 文件所在目录
    :param package: 文件所在包名
    :param exclude: 排除的文件， 默认__开头的文件都会排除
    :return:
    """
    if len(dir_path) > 0 and dir_path[0] == '~':
        dir_path = os.path.expanduser('~') + dir_path[1:]

    strategy = {}
    for root_path, _, files in os.walk(dir_path):
        if root_path.find('__') >= 0 or root_path.startswith('.'):
            continue

        package_suf = ''
        if dir_path != root_path:
            package_suf = '.' + \
                root_path[len(dir_path) + 1:].replace(os.sep, '.')

        for file_name in files:
            if not file_name.endswith('.py'):
                continue

            if file_name.startswith('__') or file_name.startswith('.') or file_name in exclude:
                continue
            module_str = '{}.{}'.format(package + package_suf, file_name[:-3])
            if module_str.startswith('.'):
                module_str = module_str[1:]
            module = importlib.import_module(module_str)

            file_names = file_name[:-3].split('_')
            name_list = [file_name.capitalize() for file_name in file_names]
            cls_name = ''.join(name_list)
            cls = module.__getattribute__(cls_name)
            if cls is not None:
                suffix = package_suf
                if len(suffix) > 0:
                    suffix = suffix[1:] + '.'
                strategy[suffix + cls_name] = cls
            else:
                print(
                    'warning: file {} not following strategy naming convention'.format(root_path + os.sep + file_name))

    return strategy


def run_until_complete(*coro):
    loop = None
    try:
        loop = asyncio.get_event_loop()
        return loop.run_until_complete(asyncio.gather(*coro))
    finally:
        if loop is not None:
            loop.close()


def is_alive(pid):
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    else:
        return True


def setup_logger(level: str):
    level = logging.getLevelName(level.upper())
    FORMAT = '[%(asctime)-15s][%(filename)s:%(lineno)d][%(name)s][%(levelname)s] %(message)s'
    logging.basicConfig(encoding='utf-8', format=FORMAT, level=level)
