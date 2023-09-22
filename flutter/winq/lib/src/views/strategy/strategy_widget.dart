import 'package:flutter/material.dart';
import 'package:winq/src/components/tab_title.dart';
import 'package:winq/src/components/tree.dart';
import 'package:winq/src/views/strategy/backtest_layout.dart';
import 'package:winq/src/views/strategy/backtest_object.dart';
import 'package:winq/src/views/strategy/backtest_widget.dart';

class StrategyWidget extends StatefulWidget {
  final TreeNode node;
  const StrategyWidget({super.key, required this.node});

  @override
  State<StrategyWidget> createState() => _StrategyWidgetState();
}

class _StrategyWidgetState extends State<StrategyWidget>
    with AutomaticKeepAliveClientMixin {
  late BacktestObject obj;

  PageController tabPageController = PageController(keepPage: true);

  late TreeNode node;
  @override
  void initState() {
    super.initState();
    node = widget.node;

    obj = BacktestObject(
        tabIndex: -1, tabTitle: [], tabWidget: [], tabWidgetLayout: []);

    _newBacktestContext();
  }

  @override
  void dispose() {
    super.dispose();
    tabPageController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;

    return Column(
      children: [
        Expanded(child: _buildBacktestWidget()),
        Divider(
          thickness: 1,
          height: 1,
          color: dividerColor,
        ),
        _buildBacktestTabTitle(),
      ],
    );
  }

  Widget _buildBacktestWidget() {
    return PageView.builder(
        controller: tabPageController,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: obj.tabTitle.length,
        itemBuilder: (context, index) {
          BacktestLayout layout = obj.tabWidgetLayout[index];
          return BacktestWidget(node: node, layout: layout);
        });
  }

  Widget _buildBacktestTabTitle() {
    return Row(
      children: [
        GestureDetector(
          onTap: () {
            _newBacktestContext();
            tabPageController.jumpToPage(obj.tabIndex);
            setState(() {});
          },
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 5.0, horizontal: 5.0),
            child: const Icon(
              Icons.add_outlined,
              size: 18.0,
            ),
          ),
        ),
        Expanded(
          child: SizedBox(
            height: 30,
            child: ListView(
              scrollDirection: Axis.horizontal,
              children: obj.tabTitle.asMap().entries.map((e) {
                return TabTitleWidget(
                    onTap: () {
                      if (e.key != obj.tabIndex) {
                        obj.tabIndex = e.key;
                        tabPageController.jumpToPage(obj.tabIndex);
                        setState(() {});
                      }
                    },
                    title: e.value,
                    isActive: e.key == obj.tabIndex);
              }).toList(),
            ),
          ),
        ),
        Container(
          padding: const EdgeInsets.only(right: 15.0),
          child: Row(
            children: [
              GestureDetector(
                onTapDown: (details) async {
                  Offset offset = details.globalPosition;
                  LayoutShow? value = await showMenu(
                      context: context,
                      shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(5.0)),
                      position: RelativeRect.fromLTRB(
                          offset.dx, offset.dy - 140.0, offset.dx, offset.dy),
                      items: [
                        PopupMenuItem<LayoutShow>(
                          mouseCursor: SystemMouseCursors.basic,
                          height: 20.0,
                          value: LayoutShow.topLeft,
                          child: Container(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(
                              '保留左侧',
                              style: TextStyle(
                                  color: Colors.white.withOpacity(0.8),
                                  fontSize: 14.0),
                            ),
                          ),
                        ),
                        PopupMenuItem<LayoutShow>(
                          mouseCursor: SystemMouseCursors.basic,
                          height: 20.0,
                          value: LayoutShow.bottomRight,
                          child: Container(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(
                              '保留右侧',
                              style: TextStyle(
                                  color: Colors.white.withOpacity(0.8),
                                  fontSize: 14.0),
                            ),
                          ),
                        ),
                        PopupMenuItem<LayoutShow>(
                          mouseCursor: SystemMouseCursors.basic,
                          height: 20.0,
                          value: LayoutShow.both,
                          child: Container(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(
                              '全部显示',
                              style: TextStyle(
                                  color: Colors.white.withOpacity(0.8),
                                  fontSize: 14.0),
                            ),
                          ),
                        ),
                      ],
                      elevation: 8.0);
                  if (value != null) {
                    BacktestLayout layout = obj.tabWidgetLayout[obj.tabIndex];
                    if (layout.horizontal != value) {
                      setState(() {
                        obj.tabWidgetLayout[obj.tabIndex] = BacktestLayout(
                            horizontal: value, vertical: layout.vertical);
                      });
                    }
                  }
                },
                child: Container(
                  padding: const EdgeInsets.symmetric(
                      vertical: 5.0, horizontal: 5.0),
                  child: const Icon(
                    Icons.horizontal_distribute_outlined,
                    size: 15.0,
                  ),
                ),
              ),
              GestureDetector(
                onTapDown: (details) async {
                  Offset offset = details.globalPosition;
                  LayoutShow? value = await showMenu(
                      context: context,
                      shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(5.0)),
                      position: RelativeRect.fromLTRB(
                          offset.dx, offset.dy - 140.0, offset.dx, offset.dy),
                      items: [
                        PopupMenuItem<LayoutShow>(
                          mouseCursor: SystemMouseCursors.basic,
                          height: 20.0,
                          value: LayoutShow.topLeft,
                          child: Container(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(
                              '保留上侧',
                              style: TextStyle(
                                  color: Colors.white.withOpacity(0.8),
                                  fontSize: 14.0),
                            ),
                          ),
                        ),
                        PopupMenuItem<LayoutShow>(
                          mouseCursor: SystemMouseCursors.basic,
                          height: 20.0,
                          value: LayoutShow.bottomRight,
                          child: Container(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(
                              '保留下侧',
                              style: TextStyle(
                                  color: Colors.white.withOpacity(0.8),
                                  fontSize: 14.0),
                            ),
                          ),
                        ),
                        PopupMenuItem<LayoutShow>(
                          mouseCursor: SystemMouseCursors.basic,
                          height: 20.0,
                          value: LayoutShow.both,
                          child: Container(
                            padding: const EdgeInsets.all(8.0),
                            child: Text(
                              '全部显示',
                              style: TextStyle(
                                  color: Colors.white.withOpacity(0.8),
                                  fontSize: 14.0),
                            ),
                          ),
                        ),
                      ],
                      elevation: 8.0);
                  if (value != null) {
                    var layout = obj.tabWidgetLayout[obj.tabIndex];
                    if (layout.vertical != value) {
                      setState(() {
                        obj.tabWidgetLayout[obj.tabIndex] = BacktestLayout(
                            horizontal: layout.horizontal, vertical: value);
                      });
                    }
                  }
                },
                child: Container(
                  padding: const EdgeInsets.symmetric(
                      vertical: 5.0, horizontal: 5.0),
                  child: const Icon(
                    Icons.vertical_distribute_outlined,
                    size: 15.0,
                  ),
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  void _newBacktestContext() {
    obj.tabIndex += 1;
    obj.tabTitle.add('测试#${obj.tabIndex + 1}');
    obj.tabWidgetLayout.add(const BacktestLayout(
        horizontal: LayoutShow.both, vertical: LayoutShow.both));
  }

  @override
  bool get wantKeepAlive => true;
}
