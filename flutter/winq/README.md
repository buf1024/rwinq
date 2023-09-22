# Trouble Shoot

- Flutter widget只要不显示在界面上，就会销毁了，所以保存实例起来是没用的，都会销毁，状态不会保存下来。

    如果要保存下来，需要用PageView之类的形式，并且widget要with AutomaticKeepAliveClientMixin这个Mixin。但这个Mixin只有用于类似PageView才有用。又或者使用IndexStack之类的组件。又或者使用状态管理。

- 由下至上传递状态用Notification，由上至下传递状态用InheritedWidget