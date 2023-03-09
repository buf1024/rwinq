import 'package:flutter/material.dart';
import 'package:hiq/src/app/iconfont.dart';
import 'package:hiq/src/app/nav.dart';
import 'package:hiq/src/components/nav_bar.dart';
import 'package:hiq/src/components/status_bar.dart';
import 'package:hiq/src/components/title_bar.dart';
import 'package:hiq/src/lock_page.dart';
import 'package:hiq/src/views/config.dart';
import 'package:hiq/src/views/dashboard.dart';
import 'package:hiq/src/views/data.dart';
import 'package:hiq/src/views/favorite.dart';
import 'package:hiq/src/views/research.dart';
import 'package:hiq/src/views/strategy.dart';
import 'package:hiq/src/views/trade.dart';
import 'package:window_manager/window_manager.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final PageController pageController = PageController();
  late NavItem actNavItem;
  Color navColor = Colors.red;
  double navContentWidth = 120;

  List<NavItem> topNavTabs = [
    NavItem(
        type: NavType.dashboard,
        pos: NavPos.top,
        label: '主页',
        iconData: IconFont.home),
    NavItem(
        type: NavType.data,
        pos: NavPos.top,
        label: '数据',
        iconData: IconFont.data),
    NavItem(
        type: NavType.research,
        pos: NavPos.top,
        label: '投研',
        iconData: IconFont.python),
    NavItem(
        type: NavType.strategy,
        pos: NavPos.top,
        label: '策略',
        iconData: IconFont.strategy),
    NavItem(
        type: NavType.favorite,
        pos: NavPos.top,
        label: '自选',
        iconData: IconFont.favorite),
    NavItem(
        type: NavType.trade,
        pos: NavPos.top,
        label: '交易',
        iconData: IconFont.trade),
  ];

  @override
  void initState() {
    super.initState();

    actNavItem = topNavTabs[0];
  }
  @override
  void dispose() {
    super.dispose();
    pageController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // backgroundColor: backgroundColor,
      body: Column(
        children: [
          TitleBar(
            child: const Text('the title'),
            onConfigCall: () async {
              Size size = await windowManager.getSize();
              onShowConfigDialog(size.width - 80.0, size.height - 80.0);
            },
            onLockCall: () => onLockScreen(),
          ),
          Expanded(
            child: _buildBody(context),
          ),
          StatusBar(
            onStatusTap: (type) => onStatusTap(type),
          )
        ],
      ),
    );
  }

  Widget _buildBody(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        NavBar(
          topNavTabs: topNavTabs,
          onTap: (value) => _onNavTabTap(value),
          actType: actNavItem.type,
        ),
        VerticalDivider(
          width: 1,
          thickness: 1,
          color: Theme.of(context).dividerColor,
        ),
        Expanded(child: _buildNavContent(context))
      ],
    );
  }

  Widget _buildNavContent(BuildContext context) {
    return PageView(
      controller: pageController,
      // onPageChanged: (index) {
      //   var item = topNavTabs[index];
      //   if (item.type != actNavItem.type) {
      //     setState(() {
      //       actNavItem = item;
      //     });
      //   }
      // },
      physics: const NeverScrollableScrollPhysics(),
      children: const [
        DashboardView(),
        DataSyncView(),
        ResearchView(),
        StrategyView(),
        FavoriteView(),
        TradeView(),
      ],
    );
  }

  void _onNavTabTap(NavItem item) {
    int index = topNavTabs.indexOf(item);
    if (index >= 0) {
      pageController.jumpToPage(index);
      setState(() {
        actNavItem = item;
      });
    }
  }

  void onStatusTap(NavType type) async {
    int index = topNavTabs.indexWhere((item) => item.type == type);
    if (index >= 0) {
      pageController.jumpToPage(index);
      setState(() {
        actNavItem = topNavTabs[index];
      });
    } else {
      if (type == NavType.notification) {
        Size size = await windowManager.getSize();
        onShowNotificationDialog(size.width - 80.0, size.height - 80.0);
      }
    }
  }

  void onLockScreen() {
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (context) => const LockPage(),
      ),
    );
  }

  void onShowNotificationDialog(double width, double height) {
    onShowConfigDialog(width, height);
  }

  void onShowConfigDialog(double width, double height) {
    showDialog(
        context: context,
        barrierDismissible: true,
        builder: (BuildContext context) {
          return Dialog(
            backgroundColor: Colors.blue.withOpacity(0.8),
            elevation: 2.0,
            shape: const RoundedRectangleBorder(
                borderRadius: BorderRadius.all(Radius.circular(10.0))),
            child: SizedBox(
              width: width,
              height: height,
              child: Column(
                children: <Widget>[
                  Container(
                    height: 32.0,
                    decoration: const BoxDecoration(
                      color: Colors.orangeAccent,
                      borderRadius: BorderRadius.only(
                          topLeft: Radius.circular(10.0),
                          topRight: Radius.circular(10.0)),
                    ),
                    alignment: Alignment.center,
                    child: const Text(
                      '系统参数配置',
                      style: TextStyle(color: Colors.white),
                    ),
                  ),
                  const Expanded(child: ConfigView()),
                  ButtonBar(
                    children: <Widget>[
                      MaterialButton(
                        color: Colors.purple,
                        onPressed: () {
                          Navigator.of(context).pop();
                        },
                        child: const Text(
                          '取消',
                          style: TextStyle(color: Colors.white),
                        ),
                      ),
                      MaterialButton(
                        color: Colors.red,
                        onPressed: () {
                          Navigator.of(context).pop();
                        },
                        child: const Text(
                          '确定',
                          style: TextStyle(color: Colors.white),
                        ),
                      ),
                    ],
                  )
                ],
              ),
            ),
          );
        });
  }
}
