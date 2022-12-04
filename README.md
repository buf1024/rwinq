python 版本的 bbq trader 执行效率太慢，用rust改写，并增加gui操作界面。


从执行效率角度来看，策略应该rust编写。可Python的编写效率快，所以策略支持python编写。


发现一些问题： 

- 国内docker镜像源太久没同步了，比官方的足足迟了一年多没同步（比如：clickhouse）。

- clickhouse的docker虚拟机占用cpu资源非常高，只要开启cpu就疯狂的转（配置低的机器，还真玩不起这些高端的东西）。

- clickhouse的两个库，一个clickhouse_rs很久没维护了，用了很多过时的api，对Date<Tz>处理怎么搞都不对。
  另外一个库clickhouse.rs如果库表的结构稍微复杂一点（甚至很简单的表），就无法出来报错。
  已经提issue（https://github.com/loyd/clickhouse.rs/issues/49），这些问题主要的原因，就是没有官方维护的库。

- reqwest 的block client和tokio异步出来会core

- pyo3-asyncio 调用时，记得`let locals = pyo3_asyncio::tokio::get_current_locals(py)?;`保存下来复用，否则会报no runtime event loop之类的错误
