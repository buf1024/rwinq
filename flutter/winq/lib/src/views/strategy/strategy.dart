import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:winq/src/components/nav_bar.dart';
import 'package:winq/src/components/tab_title.dart';
import 'package:winq/src/components/tree.dart';
import 'package:winq/src/views/strategy/strategy_widget.dart';

const kMinTreeWidth = 120.0;

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
  int tabIndex = -1;
  PageController tabPageController = PageController(keepPage: true);

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
    tabPageController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;

    return Row(children: [
      _buildTree(dividerColor),
      Expanded(
        child: Row(
          children: [
            _buildResizeDiv(dividerColor),
            // tabIndex == -1
            //     ? Container()
            //     :
            Expanded(
              child: Column(
                children: [
                  _buildTabTitle(),
                  Divider(
                    thickness: 1,
                    height: 1,
                    color: dividerColor,
                  ),
                  Expanded(
                    child: _buildBacktestWidget(),
                  ),
                ],
              ),
            ),
          ],
        ),
      )
    ]);
  }

  Widget _buildBacktestWidget() {
    return PageView.builder(
        controller: tabPageController,
        physics: const NeverScrollableScrollPhysics(),
        itemCount: tabTitle.length,
        itemBuilder: (context, index) {
          return StrategyWidget(
            node: tabTitle[index],
          );
        });
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

  Widget _buildTree(Color dividerColor) {
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

  void _onStrategySelected(TreeNode node) {
    if (node.isLeaf) {
      int index = tabTitle.indexOf(node);
      if (index < 0) {
        tabTitle.add(node);

        tabIndex = tabTitle.length - 1;
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
      tabPageController.jumpToPage(tabIndex);
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
      tabPageController.jumpToPage(tabIndex);
    }
  }

  void _onTabClose(TreeNode node, int index) {}

  @override
  bool get wantKeepAlive => true;
}
