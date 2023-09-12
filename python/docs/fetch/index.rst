数据获取
========

数据获取模块\ `pywqfetch <https://github.com/buf1024/rwinq/tree/main/python/pywqfetch>`__\ 是相对独立的模块，主要功能是从公开的数据源获取数据。\ `pywqfetch <https://github.com/buf1024/rwinq/tree/main/python/pywqfetch>`__\ 提供两种api，一种是异步版本，以`fetch_`​开头，另外一种是阻塞版本，以\ ``block_fetch``\ ​开头。

Rust编写的模块和函数是通过
``pywqfetch.pywqfetch``\ ​导出，该包原样导出Rust模块和函数(``.pyi``\ ​文件)。一般情况下，不需要使用该包下面的函数，在\ ``pywqfetch.fetch``\ ​下面有此包的包括类(``Fetch/BlockFetch``\ ​)，将Rust所有的功能汇聚成一个包装类。包装类除了可返回原始的Rust输出外，也提供可选装换为\ ``pd.Dataframe``\ ​格式的参数，同时为了清晰，有多个参数情况下，仅支持命名的方式传递参数。为了更加方便，包生成了一个全局的\ ``Fetch/BlockFetch``\ ​实例，可以直接通过\ ``fetch/block_fetch``\ ​函数使用。

如\ ``ipython``\ ​：

.. code:: python

   In [1]: import pywqfetch as fetch
      ...: await fetch.fetch_trade_date()
   Out[1]: 
         trade_date
   0       20120104
   1       20120105
   2       20120106
   3       20120109
   4       20120110
   ...          ...
   8064    20021225
   8065    20021226
   8066    20021227
   8067    20021230
   8068    20021231

   [8069 rows x 1 columns]

直接通过pip安装即可:

.. code:: shell

   pip install -U pywqfetch

.. toctree::
   :maxdepth: 2

   general
   stock
   fund
   bond


