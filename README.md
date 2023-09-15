rwinq 是Rust WINer Quatitative的缩写，当然名字是什么并不重要。

python 版本的 bbq/hiq/winq trader等，无非就是执行效率太慢，所有用传说中的rust改写，并当然也增加gui操作界面。不过，也不想放弃python的易用性，所有rust的所有代码都有会通过pyo3进行用pyhton进行绑定。

部分python代码的文档参考(Rust 文档通过`cargo doc`查看): (pywinq文档)[https://rwinq.readthedocs.io/zh_CN/latest/]


从执行效率角度来看，策略应该rust编写。可Python的编写效率快，所以策略支持python编写。


发现一些问题： 

- 国内docker镜像源太久没同步了，比官方的足足迟了一年多没同步（比如：clickhouse）。

- clickhouse的docker虚拟机占用cpu资源非常高，只要开启cpu就疯狂的转（配置低的机器，还真玩不起这些高端的东西）。

- clickhouse的两个库，一个clickhouse_rs很久没维护了，用了很多过时的api，对Date<Tz>处理怎么搞都不对。
  另外一个库clickhouse.rs如果库表的结构稍微复杂一点（甚至很简单的表），就无法出来报错。
  已经提issue（https://github.com/loyd/clickhouse.rs/issues/49），这些问题主要的原因，就是没有官方维护的库。

- reqwest 的block client和tokio异步出来会core

- pyo3-asyncio 调用时，记得`let locals = pyo3_asyncio::tokio::get_current_locals(py)?;`保存下来复用，否则会报no runtime event loop之类的错误


GUI的一个问题：

用JS还是Flutter？Flutter的强势在于可以开发手机端，弱势是在于桌面端。而且，Google在桌面端的不给力。JS的强势在于天生的易用性以及丰富的生态，已经有太多开发桌面端成功的案例。可是并不适合做手机端。然而这个注定只桌面端使用的，选择显而易见？不过貌似不太熟悉CSS呀……
