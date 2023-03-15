import 'package:data_table_2/data_table_2.dart';
import 'package:flutter/material.dart';
import 'package:hiq/src/components/tab_title.dart';

const kMinRightWidth = 80.0;
const kMinLeftWidth = 250.0;

class FavoriteView extends StatefulWidget {
  const FavoriteView({super.key});

  @override
  State<FavoriteView> createState() => _FavoriteViewState();
}

class _FavoriteViewState extends State<FavoriteView>
    with AutomaticKeepAliveClientMixin {
  double rightWidth = 300.0;
  bool isHideInfo = false;
  List<String> tabTitle = [];
  List<Widget> tabWidget = [];
  int tabIndex = 0;

  @override
  void initState() {
    super.initState();
    rightWidth = 300.0;
    isHideInfo = false;

    tabTitle = ['我们的自信', '她的世界', '你的故事', '世界的选择'];
    tabIndex = 0;
  }

  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;

    return Row(
      children: [
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              _buildLeftTabTitle(),
              Divider(
                thickness: 1,
                height: 1,
                color: dividerColor,
              ),
              _buildLeftStat(),
              Divider(
                thickness: 1,
                height: 1,
                color: dividerColor,
              ),
              Expanded(child: _buildTabContent())
            ],
          ),
        ),
        isHideInfo
            ? Container()
            : SizedBox(
                width: rightWidth,
                child: Row(
                  children: [
                    _buildResizeDiv(dividerColor),
                    Expanded(
                      child: Container(
                        color: Colors.amber.shade500,
                        child: ElevatedButton(
                            child: const Text('what the fuck!'),
                            onPressed: () {}),
                      ),
                    )
                  ],
                ),
              )
      ],
    );
  }

  Widget _buildResizeDiv(Color dividerColor) {
    double canvasWidth = MediaQuery.of(context).size.width;
    return MouseRegion(
      cursor: SystemMouseCursors.resizeColumn,
      child: GestureDetector(
        onPanUpdate: (details) {
          double newWidth = rightWidth - details.delta.dx;
          if (newWidth > kMinRightWidth &&
              newWidth + kMinLeftWidth <= canvasWidth) {
            setState(() {
              rightWidth = newWidth;
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

  Widget _buildLeftStat() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 4.0, vertical: 10.0),
      child: Wrap(
        children: [
          Container(
              decoration: BoxDecoration(
                  color: Colors.grey.withOpacity(0.8),
                  borderRadius: BorderRadius.circular(4.0)),
              padding: const EdgeInsets.all(2.0),
              child: Text.rich(TextSpan(children: [
                TextSpan(
                    text: '上证',
                    style: TextStyle(
                        fontSize: 14.0, color: Colors.white.withOpacity(0.8))),
                TextSpan(
                    text: ' +0.55%',
                    style: TextStyle(
                        fontWeight: FontWeight.bold,
                        fontSize: 14.0,
                        color: Colors.red.withOpacity(0.8)))
              ])))
        ],
      ),
    );
  }

  Widget _buildLeftTabTitle() {
    return Row(
      children: [
        Expanded(
          child: SizedBox(
            height: 30,
            child: ListView(
                scrollDirection: Axis.horizontal,
                children: tabTitle.asMap().entries.map((e) {
                  return TabTitleWidget(
                    title: e.value,
                    isActive: e.key == tabIndex,
                    onTap: () {
                      setState(() {
                        tabIndex = e.key;
                      });
                    },
                    onCloseTap: () {
                      if (tabTitle.length > 1) {
                        String title = tabTitle[tabIndex];

                        tabTitle.removeAt(e.key);
                        tabWidget.removeAt(e.key);

                        int index =
                            tabTitle.indexWhere((element) => element == title);
                        setState(() {
                          if (index == -1) {
                            tabIndex = e.key <= 0 ? 0 : e.key - 1;
                          } else {
                            tabIndex = index;
                          }
                        });
                      }
                    },
                    onSecondaryTap: (offset) {
                      showMenu(
                          context: context,
                          shape: RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(5.0)),
                          // surfaceTintColor: Colors.grey.withOpacity(0.8),

                          position: RelativeRect.fromLTRB(offset.dx, offset.dy,
                              offset.dx + 150, offset.dy + 150),
                          items: [
                            PopupMenuItem<Never>(
                                mouseCursor: SystemMouseCursors.basic,
                                height: 20.0,
                                child: Container(
                                  padding: const EdgeInsets.all(8.0),
                                  child: Row(
                                    mainAxisAlignment: MainAxisAlignment.start,
                                    children: [
                                      const Icon(
                                        Icons.remove_outlined,
                                        size: 18.0,
                                      ),
                                      const SizedBox(width: 8.0),
                                      Text(
                                        '关闭当前',
                                        style: TextStyle(
                                            color:
                                                Colors.white.withOpacity(0.8),
                                            fontSize: 14.0),
                                      )
                                    ],
                                  ),
                                )),
                            PopupMenuItem<Never>(
                                mouseCursor: SystemMouseCursors.basic,
                                height: 20.0,
                                enabled: e.key >= 1,
                                child: Container(
                                  padding: const EdgeInsets.all(8.0),
                                  child: Row(
                                    mainAxisAlignment: MainAxisAlignment.start,
                                    children: [
                                      const Icon(
                                        Icons.disabled_by_default_outlined,
                                        size: 18.0,
                                      ),
                                      const SizedBox(width: 8.0),
                                      Text(
                                        '关闭左侧',
                                        style: TextStyle(
                                            color:
                                                Colors.white.withOpacity(0.8),
                                            fontSize: 14.0),
                                      )
                                    ],
                                  ),
                                )),
                            PopupMenuItem<Never>(
                                mouseCursor: SystemMouseCursors.basic,
                                height: 20.0,
                                enabled: e.key < tabTitle.length - 1,
                                child: Container(
                                  padding: const EdgeInsets.all(8.0),
                                  child: Row(
                                    mainAxisAlignment: MainAxisAlignment.start,
                                    children: [
                                      const Icon(
                                        Icons.highlight_off_outlined,
                                        size: 18.0,
                                      ),
                                      const SizedBox(width: 8.0),
                                      Text(
                                        '关闭右侧',
                                        style: TextStyle(
                                            color:
                                                Colors.white.withOpacity(0.8),
                                            fontSize: 14.0),
                                      )
                                    ],
                                  ),
                                )),
                            PopupMenuItem<Never>(
                                mouseCursor: SystemMouseCursors.basic,
                                height: 20.0,
                                child: Container(
                                  padding: const EdgeInsets.all(8.0),
                                  child: Row(
                                    mainAxisAlignment: MainAxisAlignment.start,
                                    children: [
                                      const Icon(
                                        Icons.delete_outlined,
                                        size: 18.0,
                                      ),
                                      const SizedBox(width: 8.0),
                                      Text(
                                        '关闭全部',
                                        style: TextStyle(
                                            color:
                                                Colors.white.withOpacity(0.8),
                                            fontSize: 14.0),
                                      )
                                    ],
                                  ),
                                )),
                            const PopupMenuDivider(),
                            PopupMenuItem<Never>(
                                mouseCursor: SystemMouseCursors.basic,
                                height: 20.0,
                                child: Container(
                                  padding: const EdgeInsets.all(8.0),
                                  child: Row(
                                    mainAxisAlignment: MainAxisAlignment.start,
                                    children: [
                                      const Icon(
                                        Icons.add_outlined,
                                        size: 18.0,
                                      ),
                                      const SizedBox(width: 8.0),
                                      Text(
                                        '新增',
                                        style: TextStyle(
                                            color:
                                                Colors.white.withOpacity(0.8),
                                            fontSize: 14.0),
                                      )
                                    ],
                                  ),
                                )),
                            PopupMenuItem<Never>(
                                mouseCursor: SystemMouseCursors.basic,
                                height: 20.0,
                                child: Container(
                                  padding: const EdgeInsets.all(8.0),
                                  child: Row(
                                    mainAxisAlignment: MainAxisAlignment.start,
                                    children: [
                                      const Icon(
                                        Icons.edit_outlined,
                                        size: 18.0,
                                      ),
                                      const SizedBox(width: 8.0),
                                      Text(
                                        '重命名',
                                        style: TextStyle(
                                            color:
                                                Colors.white.withOpacity(0.8),
                                            fontSize: 14.0),
                                      )
                                    ],
                                  ),
                                )),
                          ],
                          elevation: 8.0);
                    },
                  );
                }).toList()),
          ),
        ),
        GestureDetector(
          onTap: () {},
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 5.0, horizontal: 5.0),
            child: const Icon(
              Icons.add_outlined,
              size: 14.0,
            ),
          ),
        ),
        GestureDetector(
            onTap: () {
              setState(() {
                isHideInfo = !isHideInfo;
              });
            },
            child: Container(
              padding:
                  const EdgeInsets.symmetric(vertical: 5.0, horizontal: 5.0),
              child: Tooltip(
                message: isHideInfo ? '显示详细信息' : '隐藏详细信息',
                decoration: BoxDecoration(
                    color: Colors.black.withOpacity(0.2),
                    borderRadius: BorderRadius.circular(5.0)),
                textStyle: const TextStyle(color: Colors.white, fontSize: 10.0),
                verticalOffset: 10.0,
                child: const Icon(
                  Icons.info_outline,
                  size: 14.0,
                ),
              ),
            )),
      ],
    );
  }

  Widget _buildTabContent() {
    if (tabIndex < tabWidget.length) {
      return tabWidget[tabIndex];
    }
    Widget widget;
    if (tabIndex.isEven) {
      widget = _table();
    } else {
      widget = Container(
        child: Center(
          child: const Text('我的世界空空如也！'),
        ),
      );
    }
    tabWidget.add(widget);
    return widget;
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

  @override
  bool get wantKeepAlive => true;
}
