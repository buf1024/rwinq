# 简介

[pywinq](https://github.com/buf1024/rwinq/tree/main/python)是[rwinq](https://github.com/buf1024/rwinq/tree/main)的python版本，[rwinq](https://github.com/buf1024/rwinq/tree/main)(rust winner quantitative trader)是用rust编写的个人股票分析测试系统，包括数据，回测，投研，交易等多个模块。然而rust是编译型的系统语言，不便于快速建模和测试，所以这才有了这个[pywinq](https://github.com/buf1024/rwinq/tree/main/python)。[pywinq](https://github.com/buf1024/rwinq/tree/main/python)所提供的核心功能基本和[rwinq](https://github.com/buf1024/rwinq/tree/main)一致，而且比原生的python效率高出几个数量级。而且[pywinq](https://github.com/buf1024/rwinq/tree/main/python)各个模块之间先对独立，可以独立安装。不如你只对怎么获取财经数据感兴趣，只需要安装`pywqfech`​而忽略其他模块。

不过需要注意的是，这里所提供的方式方法不一定是主流股票投资的方式，也不能保证这个所谓的系统能够助力盈利，更不能保证不会亏损。这项目的功能更多是探索应用rust实现一个完整的系统而已。

另外还需要注意的是，本项目中所涉及到的财经数据，全部来源于公开的数据，不涉及个人或其他隐私数据，也不能保证所收集到的数据是正确的。所获取到的数据仅用于学习和研究用，禁止商用。

‍
