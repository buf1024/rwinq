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




# import pandas as pd
# import copy


# class ChipDistribution():

#     def __init__(self):
#         self.Chip = {} # 当前获利盘
#         self.ChipList = {}  # 所有的获利盘的

#     def get_data(self):
#         self.data = pd.read_csv('test.csv')

#     def calcuJUN(self,dateT,highT, lowT, volT, TurnoverRateT, A, minD):

#         x =[]
#         l = (highT - lowT) / minD
#         for i in range(int(l)):
#             x.append(round(lowT + i * minD, 2))
#         length = len(x)
#         eachV = volT/length
#         for i in self.Chip:
#             self.Chip[i] = self.Chip[i] *(1 -TurnoverRateT * A)
#         for i in x:
#             if i in self.Chip:
#                 self.Chip[i] += eachV *(TurnoverRateT * A)
#             else:
#                 self.Chip[i] = eachV *(TurnoverRateT * A)
#         import copy
#         self.ChipList[dateT] = copy.deepcopy(self.Chip)



#     def calcuSin(self,dateT,highT, lowT,avgT, volT,TurnoverRateT,minD,A):
#         x =[]

#         l = (highT - lowT) / minD
#         for i in range(int(l)):
#             x.append(round(lowT + i * minD, 2))

#         length = len(x)

#         #计算仅仅今日的筹码分布
#         tmpChip = {}
#         eachV = volT/length


#         #极限法分割去逼近
#         for i in x:
#             x1 = i
#             x2 = i + minD
#             h = 2 / (highT - lowT)
#             s= 0
#             if i < avgT:
#                 y1 = h /(avgT - lowT) * (x1 - lowT)
#                 y2 = h /(avgT - lowT) * (x2 - lowT)
#                 s = minD *(y1 + y2) /2
#                 s = s * volT
#             else:
#                 y1 = h /(highT - avgT) *(highT - x1)
#                 y2 = h /(highT - avgT) *(highT - x2)

#                 s = minD *(y1 + y2) /2
#                 s = s * volT
#             tmpChip[i] = s


#         for i in self.Chip:
#             self.Chip[i] = self.Chip[i] *(1 -TurnoverRateT * A)

#         for i in tmpChip:
#             if i in self.Chip:
#                 self.Chip[i] += tmpChip[i] *(TurnoverRateT * A)
#             else:
#                 self.Chip[i] = tmpChip[i] *(TurnoverRateT * A)
#         import copy
#         self.ChipList[dateT] = copy.deepcopy(self.Chip)


#     def calcu(self,dateT,highT, lowT,avgT, volT, TurnoverRateT,minD = 0.01, flag=1 , AC=1):
#         if flag ==1:
#             self.calcuSin(dateT,highT, lowT,avgT, volT, TurnoverRateT,A=AC, minD=minD)
#         elif flag ==2:
#             self.calcuJUN(dateT,highT, lowT, volT, TurnoverRateT, A=AC, minD=minD)

#     def calcuChip(self, flag=1, AC=1):  #flag 使用哪个计算方式,    AC 衰减系数
#         low = self.data['low']
#         high = self.data['high']
#         vol = self.data['volume']
#         TurnoverRate = self.data['TurnoverRate']
#         avg = self.data['avg']
#         date = self.data['date']

#         for i in range(len(date)):
#         #     if i < 90:
#         #         continue

#             highT = high[i]
#             lowT = low[i]
#             volT = vol[i]
#             TurnoverRateT = TurnoverRate[i]
#             avgT = avg[i]
#             # print(date[i])
#             dateT = date[i]
#             self.calcu(dateT,highT, lowT,avgT, volT, TurnoverRateT/100, flag=flag, AC=AC)  # 东方财富的小数位要注意，兄弟萌。我不除100懵逼了

#         # 计算winner
#     def winner(self,p=None):
#             Profit = []
#             date = self.data['date']

#             if p == None:  # 不输入默认close
#                 p = self.data['close']
#                 count = 0
#                 for i in self.ChipList:
#                     # 计算目前的比例

#                     Chip = self.ChipList[i]
#                     total = 0
#                     be = 0
#                     for i in Chip:
#                         total += Chip[i]
#                         if i < p[count]:
#                             be += Chip[i]
#                     if total != 0:
#                         bili = be / total
#                     else:
#                         bili = 0
#                     count += 1
#                     Profit.append(bili)
#             else:
#                 for i in self.ChipList:
#                     # 计算目前的比例

#                     Chip = self.ChipList[i]
#                     total = 0
#                     be = 0
#                     for i in Chip:
#                         total += Chip[i]
#                         if i < p:
#                             be += Chip[i]
#                     if total != 0:
#                         bili = be / total
#                     else:
#                         bili = 0
#                     Profit.append(bili)

#             # import matplotlib.pyplot as plt
#             # plt.plot(date[len(date) - 200:-1], Profit[len(date) - 200:-1])
#             # plt.show()

#             return Profit

#     def lwinner(self,N = 5, p=None):

#         data = copy.deepcopy(self.data)
#         date = data['date']
#         ans = []
#         for i in range(len(date)):
#             print(date[i])
#             if i < N:
#                 ans.append(None)
#                 continue
#             self.data = data[i-N:i]
#             self.data.index= range(0,N)
#             self.__init__()
#             self.calcuChip()    #使用默认计算方式
#             a = self.winner(p)
#             ans.append(a[-1])
#         import matplotlib.pyplot as plt
#         plt.plot(date[len(date) - 60:-1], ans[len(date) - 60:-1])
#         plt.show()

#         self.data = data
#         return ans



#     def cost(self,N):
#         date = self.data['date']

#         N = N / 100  # 转换成百分比
#         ans = []
#         for i in self.ChipList:  # 我的ChipList本身就是有顺序的
#             Chip = self.ChipList[i]
#             ChipKey = sorted(Chip.keys())  # 排序
#             total = 0  # 当前比例
#             sumOf = 0  # 所有筹码的总和
#             for j in Chip:
#                 sumOf += Chip[j]

#             for j in ChipKey:
#                 tmp = Chip[j]
#                 tmp = tmp / sumOf
#                 total += tmp
#                 if total > N:
#                     ans.append(j)
#                     break
#         import matplotlib.pyplot as plt
#         plt.plot(date[len(date) - 1000:-1], ans[len(date) - 1000:-1])
#         plt.show()
#         return ans



# # if __name__ == "__main__":
# #     a=ChipDistribution()
# #     a.get_data() #获取数据
# #     a.calcuChip(flag=1, AC=1) #计算
# #     a.winner() #获利盘
# #     a.cost(90) #成本分布



# #     a.lwinner()