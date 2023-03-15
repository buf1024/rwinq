import 'package:data_table_2/data_table_2.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:hiq/src/components/nav_bar.dart';
import 'package:hiq/src/components/tab_title.dart';
import 'package:hiq/src/components/tree.dart';

const kMinTreeWidth = 120.0;
const kMinResultHeight = 65.0;

class StrategyView extends StatefulWidget {
  const StrategyView({super.key});

  @override
  State<StrategyView> createState() => _StrategyViewState();
}

class _StrategyViewState extends State<StrategyView>
    with AutomaticKeepAliveClientMixin {
  double treeWidth = 150.0;
  double starTreeHeight = 100.0;
  bool isHideTree = false;
  bool isHideStar = true;
  bool isStSearchShow = false;
  bool isRefreshing = false;

  TreeNode? selectedNode;

  List<TreeNode> tabTitle = [];
  List<Widget> tabWidget = [];
  int tabIndex = -1;

  Map<String, BacktestObject> backtestMap = {};

  FocusNode stSearchEditNode = FocusNode();
  TextEditingController stSearchEditController = TextEditingController();

  TreeNode stTree = TreeNode(children: [
    TreeNode(children: const [], text: '我的世界空无一物', path: '/'),
    TreeNode(
        children: [TreeNode(text: '承认失败', path: '/我是谁？')],
        text: '我是谁？',
        path: '/'),
    TreeNode(
        children: [TreeNode(text: '神算目录', path: '/必胜')],
        text: '必胜',
        path: '/',
        expand: true),
    TreeNode(text: '100%胜率', path: '/')
  ], text: '策略', path: '', expand: true);

  String stFilteredText = '';

  @override
  void initState() {
    super.initState();
    treeWidth = 150.0;
    isHideTree = false;

    tabIndex = -1;
  }

  @override
  void dispose() {
    super.dispose();
    stSearchEditNode.dispose();
    stSearchEditController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;

    return Row(children: [
      _buildTreeSelection(dividerColor),
      Expanded(
        child: Row(
          children: [
            _buildResizeDiv(dividerColor),
            tabIndex == -1
                ? Container()
                : Expanded(
                    child: Column(
                      children: [
                        _buildTabTitle(),
                        Divider(
                          thickness: 1,
                          height: 1,
                          color: dividerColor,
                        ),
                        Expanded(child: _buildBacktestWidget()),
                        Divider(
                          thickness: 1,
                          height: 1,
                          color: dividerColor,
                        ),
                        _buildBacktestTabTitle(),
                      ],
                    ),
                  ),
          ],
        ),
      )
    ]);
  }

  Widget _buildTabTitle() {
    return Row(
      children: [
        GestureDetector(
          onTap: () {
            setState(() {
              isHideTree = !isHideTree;
            });
          },
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 5.0, horizontal: 5.0),
            child: Tooltip(
              message: isHideTree ? '显示策略树' : '隐藏策略树',
              decoration: BoxDecoration(
                  color: Colors.black.withOpacity(0.2),
                  borderRadius: BorderRadius.circular(5.0)),
              textStyle: const TextStyle(color: Colors.white, fontSize: 10.0),
              verticalOffset: 10.0,
              child: const Icon(
                Icons.psychology_outlined,
                size: 18.0,
              ),
            ),
          ),
        ),
        Expanded(
          child: SizedBox(
            height: 30,
            child: ListView(
              scrollDirection: Axis.horizontal,
              children: tabTitle.asMap().entries.map((e) {
                return TabTitleWidget(
                  title: e.value.text,
                  isActive: e.key == tabIndex,
                  onTap: () => _onTabTap(e.value, e.key),
                  onCloseTap: () => _onTabClose(e.value, e.key),
                );
              }).toList(),
            ),
          ),
        )
      ],
    );
  }

  Widget _buildBacktestTabTitle() {
    BacktestObject? obj = backtestMap[tabTitle[tabIndex].key];
    if (obj == null) {
      return Container();
    }
    return Row(
      children: [
        GestureDetector(
          onTap: () {
            _newBacktestContext(selectedNode!);
            setState(() {
              
            });
            print('tabIndex: ${obj.tabIndex}');
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
                  onTap: (){
                    if(e.key != obj.tabIndex) {
                      obj.tabIndex = e.key;
                      print('tabIndex: ${obj.tabIndex}');
                      setState(() {
                      });
                    }
                  },
                    title: e.value, isActive: e.key == obj.tabIndex);
              }).toList(),
            ),
          ),
        ),
        const Spacer(),
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
                      print('tabIndex: ${obj.tabIndex}');
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
                    var layout = obj.tabWidgetLayout[tabIndex];
                    if (layout.vertical != value) {
                      setState(() {
                        obj.tabWidgetLayout[tabIndex] = BacktestLayout(
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

  Widget _buildBacktestWidget() {
    TreeNode node = tabTitle[tabIndex];
    print('node: ${node.key}');
    BacktestObject? obj = backtestMap[node.key];
    if (obj == null) {
      return Container();
    }

    return BacktestLayoutWidget(
      layout: obj.tabWidgetLayout[obj.tabIndex],
      child: obj.tabWidget[obj.tabIndex],
    );
  }

  Widget _buildTreeSelection(Color dividerColor) {
    return isHideTree
        ? Container()
        : SizedBox(
            width: treeWidth,
            child: Column(
              children: [
                Container(
                  height: 30.0,
                  padding: const EdgeInsets.all(4.0),
                  child: Row(
                    children: [
                      Container(
                        padding: const EdgeInsets.only(right: 4.0),
                        child: GestureDetector(
                          onTap: () {
                            setState(() {
                              isHideStar = !isHideStar;
                            });
                          },
                          child: const Icon(
                            Icons.star_outline,
                            size: 18.0,
                          ),
                        ),
                      ),
                      Container(
                        padding: const EdgeInsets.only(right: 4.0),
                        child: GestureDetector(
                          onTap: () {},
                          child: const Icon(
                            Icons.add_outlined,
                            size: 18.0,
                          ),
                        ),
                      ),
                      Container(
                        padding: const EdgeInsets.only(right: 4.0),
                        child: GestureDetector(
                          onTap: () {},
                          child: const Icon(
                            Icons.remove_outlined,
                            size: 18.0,
                          ),
                        ),
                      ),
                      Container(
                        padding: const EdgeInsets.only(right: 4.0),
                        child: GestureDetector(
                          onTap: () async {
                            if (!isRefreshing) {
                              setState(() {
                                isRefreshing = true;
                              });
                              await Future.delayed(const Duration(seconds: 3));
                              setState(() {
                                isRefreshing = false;
                              });
                            }
                          },
                          child: isRefreshing
                              ? const CupertinoActivityIndicator(
                                  radius: 8,
                                )
                              : const Icon(Icons.refresh,
                                  size: 18.0, color: Colors.grey),
                        ),
                      ),
                      const Spacer(),
                      Container(
                          padding: const EdgeInsets.only(right: 4.0),
                          child: GestureDetector(
                              onTap: () {
                                isStSearchShow = !isStSearchShow;
                                if (isStSearchShow) {
                                  stSearchEditNode.requestFocus();
                                }
                                stSearchEditController.text = '';
                                setState(() {});
                              },
                              child: const Icon(
                                Icons.search_outlined,
                                size: 18.0,
                              ))),
                    ],
                  ),
                ),
                isStSearchShow
                    ? Container(
                        padding: const EdgeInsets.symmetric(
                            vertical: 4.0, horizontal: 2.0),
                        height: 30.0,
                        child: TextField(
                          controller: stSearchEditController,
                          focusNode: stSearchEditNode,
                          autofocus: true,
                          autocorrect: false,
                          obscuringCharacter: '*',
                          cursorWidth: 1.0,
                          cursorColor: Colors.grey.withOpacity(0.8),
                          style: const TextStyle(fontSize: 12.0),
                          decoration: InputDecoration(
                            contentPadding: const EdgeInsets.symmetric(
                                vertical: 2.0, horizontal: 6.0),
                            focusedBorder: OutlineInputBorder(
                                borderSide: BorderSide(
                                    color: Colors.grey.withOpacity(0.8)),
                                borderRadius: BorderRadius.circular(5)),
                            enabledBorder: OutlineInputBorder(
                                borderSide: BorderSide(
                                    color: Colors.grey.withOpacity(0.8)),
                                borderRadius: BorderRadius.circular(5)),
                          ),
                          onChanged: (value) {
                            setState(() {
                              stFilteredText = value;
                            });
                          },
                        ),
                      )
                    : Container(),
                Divider(
                  thickness: 1,
                  height: 1,
                  color: dividerColor,
                ),
                Expanded(child: _buildStrategyTree(dividerColor))
              ],
            ),
          );
  }

  Widget _buildStrategyTree(Color dividerColor) {
    return Column(
      children: [
        Expanded(
            child: TreeWidget(
          root: stTree,
          readOnly: false,
          keepEmpty: stFilteredText.isEmpty,
          filteredText: stFilteredText,
          onStared: (node) {
            setState(() {});
          },
          onSelected: (node) => _onStrategySelected(node),
        )),
        isHideStar
            ? Container()
            : MouseRegion(
                cursor: SystemMouseCursors.resizeRow,
                child: GestureDetector(
                  onPanUpdate: (details) {
                    double newWidth = starTreeHeight - details.delta.dy;
                    if (newWidth > 0) {
                      setState(() {
                        starTreeHeight = newWidth;
                      });
                    }
                  },
                  child: Column(
                    children: [
                      Divider(
                        thickness: 2,
                        height: 2,
                        color: dividerColor,
                      ),
                      SizedBox(
                        height: starTreeHeight,
                        child: TreeWidget(
                          root: stTree,
                          onSelected: (node) => _onStrategySelected(node),
                          readOnly: true,
                          keepEmpty: false,
                          filteredStar: true,
                        ),
                      )
                    ],
                  ),
                ),
              ),
      ],
    );
  }

  Widget _buildResizeDiv(Color dividerColor) {
    double canvasWidth =
        MediaQuery.of(context).size.width - kNaviBarWidth - 2.0;
    return isHideTree
        ? Container()
        : MouseRegion(
            cursor: SystemMouseCursors.resizeColumn,
            child: GestureDetector(
              onPanUpdate: (details) {
                double newWidth = treeWidth + details.delta.dx;
                if (newWidth > kMinTreeWidth && newWidth <= canvasWidth) {
                  setState(() {
                    treeWidth = newWidth;
                  });
                }
              },
              child: VerticalDivider(
                thickness: 2,
                width: 2,
                color: dividerColor,
              ),
            ),
          );
  }

  void _newBacktestContext(TreeNode node) {
    if (!backtestMap.containsKey(node.key)) {
      backtestMap[node.key] = BacktestObject(
          tabIndex: -1, tabTitle: [], tabWidget: [], tabWidgetLayout: []);
    }
    BacktestObject? obj = backtestMap[node.key];
    if (obj != null) {
      print('add node: ${node.key}');
      obj.tabIndex += 1;
      obj.tabTitle.add('测试#${obj.tabIndex + 1}');
      obj.tabWidget.add(BacktestWidget(
        node: node,
      ));
      obj.tabWidgetLayout.add(const BacktestLayout(
          horizontal: LayoutShow.both, vertical: LayoutShow.both));
    }
  }

  void _onStrategySelected(TreeNode node) {
    if (node.isLeaf) {
      int index = tabTitle.indexOf(node);
      if (index < 0) {
        tabTitle.add(node);
        tabIndex = tabTitle.length - 1;
        _newBacktestContext(node);
      } else {
        if (index != tabIndex) {
          tabIndex = index;
        }
      }
      if (selectedNode != null) {
        selectedNode!.selected = false;
      }
      selectedNode = node;

      setState(() {});
    }
  }

  void _onTabTap(TreeNode node, int index) {
    if (tabIndex != index) {
      if (selectedNode != null) {
        selectedNode!.selected = false;
      }
      selectedNode = node;
      setState(() {
        tabIndex = index;
        node.selected = true;
      });
    }
  }

  void _onTabClose(TreeNode node, int index) {}

  @override
  bool get wantKeepAlive => true;
}

class BacktestObject {
  List<String> tabTitle = [];
  List<Widget> tabWidget = [];
  List<BacktestLayout> tabWidgetLayout = [];
  int tabIndex = 0;
  BacktestObject(
      {required this.tabIndex,
      required this.tabWidget,
      required this.tabTitle,
      required this.tabWidgetLayout});
}

enum LayoutShow { topLeft, both, bottomRight }

class BacktestLayout extends Equatable {
  final LayoutShow horizontal;
  final LayoutShow vertical;

  const BacktestLayout({required this.horizontal, required this.vertical});

  @override
  List<Object?> get props => [horizontal, vertical];
}

class BacktestLayoutWidget extends InheritedWidget {
  static of(BuildContext context) =>
      context.dependOnInheritedWidgetOfExactType<BacktestLayoutWidget>();
  final BacktestLayout layout;

  const BacktestLayoutWidget(
      {super.key, required this.layout, required super.child});

  @override
  bool updateShouldNotify(covariant BacktestLayoutWidget oldWidget) {
    return oldWidget.layout != layout;
  }
}

class BacktestWidget extends StatefulWidget {
  final TreeNode node;

  const BacktestWidget({super.key, required this.node});

  @override
  State<BacktestWidget> createState() => _BacktestWidgetState();
}

class _BacktestWidgetState extends State<BacktestWidget>
// with AutomaticKeepAliveClientMixin
{
  final List<String> resultTabTitle = ['策略选择', '交易记录', '交易信号', '策略日志'];
  int resultTabIndex = 0;

  double victoryWidth = 100.0;
  double resultHeight = 150.0;

  @override
  void initState() {
    super.initState();

    resultTabIndex = 0;
    print('resultTabIndex ${resultTabIndex}');
  }

  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;
    BacktestLayout layout = BacktestLayoutWidget.of(context).layout;

    return Column(
      children: [
        layout.vertical == LayoutShow.bottomRight
            ? Container()
            : Expanded(
                flex: 1,
                child: Row(
                  children: [
                    layout.horizontal == LayoutShow.bottomRight
                        ? Container()
                        : Expanded(flex: 1, child: _buildConfig()),
                    layout.horizontal != LayoutShow.both
                        ? Container()
                        : _buildHorizontalResizeDiv(dividerColor),
                    layout.horizontal == LayoutShow.topLeft
                        ? Container()
                        : Container(
                            width: victoryWidth,
                            padding: const EdgeInsets.all(5.0),
                            child: SingleChildScrollView(
                              scrollDirection: Axis.horizontal,
                              child: _buildVictory(),
                            ),
                          )
                  ],
                ),
              ),
        layout.vertical != LayoutShow.both
            ? Container()
            : _buildVerticalResizeDiv(dividerColor),
        layout.vertical == LayoutShow.topLeft
            ? Container()
            : (layout.vertical == LayoutShow.both
                ? SizedBox(
                    height: resultHeight,
                    child: _buildResult(dividerColor),
                  )
                : Expanded(child: _buildResult(dividerColor))),
      ],
    );
  }

  Widget _buildResult(Color dividerColor) {
    return Column(
      children: [
        Divider(
          height: 1,
          color: dividerColor,
        ),
        _buildResultTabTitle(),
        Divider(
          height: 1,
          color: dividerColor,
        ),
        Expanded(child: _buildResultTabWidget()),
      ],
    );
  }

  Widget _buildResultTabWidget() {
    return IndexedStack(
      index: resultTabIndex,
      children: [
        Text('策略选择: ${widget.node.text}'),
        _table(),
        const Text('交易信号'),
        const Text('策略日志')
      ],
    );
    // if (resultTabIndex == 0) {
    //   return Text('策略选择: ${widget.node.text}');
    // } else if (resultTabIndex == 1) {
    //   return _table();
    // } else if (resultTabIndex == 2) {
    //   return const Text('交易信号');
    // } else if (resultTabIndex == 3) {
    //   return const Text('策略日志');
    // }
    // return Container();
  }

  Widget _buildHorizontalResizeDiv(Color dividerColor) {
    return MouseRegion(
      cursor: SystemMouseCursors.resizeColumn,
      child: GestureDetector(
        onPanUpdate: (details) {
          double newWidth = victoryWidth - details.delta.dx;
          if (newWidth > 0) {
            setState(() {
              victoryWidth = newWidth;
            });
          }
        },
        child: VerticalDivider(
          thickness: 2,
          width: 2,
          color: dividerColor,
        ),
      ),
    );
  }

  Widget _buildVerticalResizeDiv(Color dividerColor) {
    return MouseRegion(
      cursor: SystemMouseCursors.resizeRow,
      child: GestureDetector(
        onPanUpdate: (details) {
          double newHeight = resultHeight - details.delta.dy;
          if (newHeight >= kMinResultHeight) {
            setState(() {
              resultHeight = newHeight;
            });
          }
        },
        child: Divider(
          thickness: 2,
          height: 2,
          color: dividerColor,
        ),
      ),
    );
  }

  Widget _buildConfig() {
    return Text('config: ${widget.node.text}');
  }

  Widget _buildVictory() {
    return const Text('victory');
  }

  Widget _buildResultTabTitle() {
    return Row(
      children: [
        Expanded(
          child: SizedBox(
            height: 30,
            child: ListView(
              scrollDirection: Axis.horizontal,
              children: resultTabTitle.asMap().entries.map((e) {
                return TabTitleWidget(
                  onTap: () {
                    if (e.key != resultTabIndex) {
                      setState(() {
                        print('key: ${e.key}');
                        resultTabIndex = e.key;
                      });
                    }
                  },
                  title: e.value,
                  isActive: e.key == resultTabIndex,
                  noPrefix: true,
                  noClose: true,
                );
              }).toList(),
            ),
          ),
        )
      ],
    );
  }

  Widget _table() {
    return Padding(
      padding: const EdgeInsets.all(1),
      child: DataTable2(
        columnSpacing: 12,
        horizontalMargin: 12,
        dividerThickness:
            1, // this one will be ignored if [border] is set above
        bottomMargin: 10,
        minWidth: 900,
        sortColumnIndex: 4,
        sortAscending: true,
        sortArrowIcon: Icons.keyboard_arrow_up, // custom arrow
        sortArrowAnimationDuration: const Duration(milliseconds: 500),
        dataRowHeight: 20.0,
        headingRowHeight: 30.0,
        fixedLeftColumns: 1,
        headingRowColor: MaterialStateProperty.resolveWith(
            (states) => Colors.grey.withOpacity(0.8)),
        columns: [
          DataColumn2(
              label: Container(
            decoration: BoxDecoration(color: Colors.grey.withOpacity(0.8)),
            child: const Text('id'),
          )),
          DataColumn2(label: Text('代码')),
          DataColumn2(label: Text('名称'), fixedWidth: 50.0),
          DataColumn2(label: Text('id')),
          DataColumn2(label: Text('code')),
          DataColumn2(label: Text('这个是名称')),
          DataColumn2(label: Text('id')),
          DataColumn2(label: Text('code')),
          DataColumn2(label: Text('这个是名称')),
          DataColumn2(label: Text('id')),
          DataColumn2(label: Text('code')),
          DataColumn2(label: Text('这个是名称')),
        ],
        rows: [
          ...List.generate(100, (v) => v).map(
            (e) {
              return DataRow2(
                color: e.isEven
                    ? MaterialStateProperty.all(
                        Colors.lightBlue.withOpacity(0.8))
                    : null,
                cells: [
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                  DataCell(Text('#$e 天神我才')),
                ],
              );
            },
          ).toList()
        ],
      ),
    );
  }

  // @override
  // bool get wantKeepAlive => true;
}
