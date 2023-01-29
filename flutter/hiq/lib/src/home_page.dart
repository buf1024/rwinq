import 'package:flutter/material.dart';
import 'package:hiq/src/app/constants.dart';
import 'package:hiq/src/app/iconfont.dart';
import 'package:hiq/src/components/nav_bar.dart';
import 'package:hiq/src/components/status_bar.dart';
import 'package:hiq/src/components/title_bar.dart';
import 'package:hiq/src/views/config_view.dart';
import 'package:hiq/src/views/data_sync_view.dart';
import 'package:hiq/src/views/favorite_view.dart';
import 'package:hiq/src/views/strategy_view.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  late NavItem actNavItem;
  Color navColor = Colors.red;
  double navContentWidth = 120;

  List<NavItem> topNavTabs = [
    NavItem(
        type: NavType.data,
        pos: NavPos.top,
        tooltip: '数据',
        iconData: IconFont.data),
    NavItem(
        type: NavType.analyze,
        pos: NavPos.top,
        tooltip: '投研',
        iconData: IconFont.python),
    NavItem(
        type: NavType.strategy,
        pos: NavPos.top,
        tooltip: '策略',
        iconData: IconFont.celve_yonghucelve),
    NavItem(
        type: NavType.favorite,
        pos: NavPos.top,
        tooltip: '自选',
        iconData: IconFont.favorite),
    NavItem(
        type: NavType.trade,
        pos: NavPos.top,
        tooltip: '交易',
        iconData: IconFont.jiaoyiguanli),
  ];
  List<NavItem> bottomNavTabs = [
    NavItem(
        type: NavType.config,
        pos: NavPos.bottom,
        tooltip: '配置',
        iconData: IconFont.config),
  ];

  @override
  void initState() {
    super.initState();

    actNavItem = topNavTabs[0];
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // backgroundColor: backgroundColor,
      body: Column(
        children: [
          const TitleBar(
            child: Text('the title'),
          ),
          Expanded(
            child: _buildBody(context),
          ),
          const StatusBar(
            child: Text('status'),
          )
        ],
      ),
    );
  }

  Widget _buildBody(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _buildNavBar(context),
        Expanded(child: _buildNavContent(context))
      ],
    );
  }

  Widget _buildNavContent(BuildContext context) {
    switch(actNavItem.type) {
      case NavType.data: return const DataSyncView();
      case NavType.strategy: return const StrategyView();
      case NavType.analyze: return Container(child: Text('analyze'),);
      case NavType.quotation: return Container(child: Text('quotation'),);
      case NavType.trade: return Container(child: Text('trade'),);
      case NavType.favorite: return const FavoriteView();
      case NavType.config: return const ConfigView();
    }
  }

  Widget _buildNavBar(BuildContext context) {
    return Container(
      width: kNavBarWidth,
      child: NavBar(
          topNavTabs: topNavTabs,
          bottomNavTabs: bottomNavTabs,
          onTap: (value) => _onNavTabTap(value),
          actType: actNavItem.type),
    );
  }

  void _onNavTabTap(NavItem item) {
    setState(() {
      actNavItem = item;
    });
  }
}
