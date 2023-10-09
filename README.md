由于机器太卡，无法正常编译rust代码，等赚钱买新机器在继续折腾。

===============
rwinq 是Rust WINer Quatitative的缩写，当然名字是什么并不重要。

python 版本的 bbq/hiq/winq trader等，无非就是执行效率太慢，所有用传说中的rust改写，并当然也增加gui操作界面。不过，也不想放弃python的易用性，所有rust的所有代码都有会通过pyo3进行用pyhton进行绑定。

部分python代码的文档参考(Rust 文档通过`cargo doc`查看): (pywinq文档)[https://rwinq.readthedocs.io/zh_CN/latest/]


从执行效率角度来看，策略应该rust编写。可Python的编写效率快，所以策略支持python编写。


- reqwest 的block client和tokio异步出来会core

- pyo3-asyncio 调用时，记得`let locals = pyo3_asyncio::tokio::get_current_locals(py)?;`保存下来复用，否则会报no runtime event loop之类的错误


GUI的一个问题：

用JS还是Flutter？Flutter的强势在于可以开发手机端，弱势是在于桌面端。而且，Google在桌面端的不给力。JS的强势在于天生的易用性以及丰富的生态，已经有太多开发桌面端成功的案例。可是并不适合做手机端。然而这个注定只桌面端使用的，选择显而易见？不过貌似不太熟悉CSS呀……

### 用户模型

一个用户有n个条交易策略，一条交易策略对应一个交易账户，同时配置一条风控策略，一个交易前端(broker)。
```
user(1)->strategy(n)->account(1)
                    ->risk(1)
                    ->broker(1)
```

模块划分:

行情模块。
通过rabbitmq进行交互，功能如下

1. 订阅行情。
2. 下发行情。直接丢都队列, account监听队列。account要处理最新的


交易模块
交易模块由行情驱动，account订阅和监听行情队列。account注意要处理最新的的行情，避免处理不及时而处理了积压的行情。
broker为券商，登录券商交易端后，需要同步交易信息。
行情到达account后，做常规的更新后，调用对应的策略，策略计算后触发交易信号。
交易信号经过风控拦截计算后，发给broker，同时broker反馈相应结果给account
risk定时监控account,同时发出signal给account，由account发平仓给broker
需要提供人工干预的接口
```
quotation -> account -> strategy
                     -> risk
          
           broker -> signal <-> broker 
        risk
```

web server 模块的统一入口，jupyter分析，行情, websocket通讯

数据存储模块，提供缺失数据及时补全

task executor

broker独立docker部署

select模块

以上独立docker部署。