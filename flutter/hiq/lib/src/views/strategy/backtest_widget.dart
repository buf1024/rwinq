import 'package:data_table_2/data_table_2.dart';
import 'package:flutter/material.dart';
import 'package:hiq/src/components/tab_title.dart';
import 'package:hiq/src/components/tree.dart';
import 'package:hiq/src/views/strategy/backtest_layout.dart';

const kMinResultHeight = 65.0;

class BacktestWidget extends StatefulWidget {
  final TreeNode node;
  final BacktestLayout layout;

  const BacktestWidget({super.key, required this.node, required this.layout});

  @override
  State<BacktestWidget> createState() => _BacktestWidgetState();
}

class _BacktestWidgetState extends State<BacktestWidget>
    with AutomaticKeepAliveClientMixin {
      
  final List<String> resultTabTitle = ['策略选择', '交易记录', '交易信号', '策略日志'];
  int resultTabIndex = 0;

  double victoryWidth = 100.0;
  double resultHeight = 150.0;

  late BacktestLayout layout;
  late TreeNode node;

  @override
  void initState() {
    super.initState();

    resultTabIndex = 0;
    layout = widget.layout;
    node = widget.node;
_stateFromNode(node);
  }

  @override
  void didUpdateWidget(covariant BacktestWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.layout != widget.layout) {
      layout = widget.layout;
    }
    if(oldWidget.node != widget.node) {
      node = widget.node;
      _stateFromNode(node);
    }
  }

  void _stateFromNode(TreeNode node) {
    if (node.text == '100%胜率') {
      resultTabIndex = 2;
    }
  }

  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;
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
                        : Expanded(flex: 1, child: _buildConfig(dividerColor)),
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

  Widget _buildInputField(String text) {
    return Row(
      children: [
        text.isEmpty ? Container() : Text(text),
        Container(
          padding: const EdgeInsets.symmetric(vertical: 4.0, horizontal: 2.0),
          height: 35.0,
          width: 100.0,
          child: TextField(
            autofocus: true,
            autocorrect: false,
            cursorWidth: 1.0,
            cursorColor: Colors.grey.withOpacity(0.8),
            style: const TextStyle(fontSize: 14.0),
            decoration: InputDecoration(
              contentPadding:
                  const EdgeInsets.symmetric(vertical: 2.0, horizontal: 6.0),
              focusedBorder: OutlineInputBorder(
                  borderSide: BorderSide(color: Colors.grey.withOpacity(0.8)),
                  borderRadius: BorderRadius.circular(5)),
              enabledBorder: OutlineInputBorder(
                  borderSide: BorderSide(color: Colors.grey.withOpacity(0.8)),
                  borderRadius: BorderRadius.circular(5)),
            ),
            onChanged: (value) {},
          ),
        ),
      ],
    );
  }

  Widget _buildParam(Color dividerColor) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const SizedBox(
          width: 8.0,
        ),
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Text('公共参数:'),
            Row(
              children: [
                _buildInputField('周期: '),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 5.0),
                  child: const Text('至'),
                ),
                _buildInputField(''),
              ],
            ),
            _buildInputField('资金: '),
            _buildInputField('基准: '),
          ],
        ),
        const SizedBox(width: 2.0),
        const VerticalDivider(thickness: 10, width: 10, color: Colors.red),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              const Text('自定义参数:'),
              Row(
                children: [
                  _buildInputField('基准: '),
                  const SizedBox(width: 4.0),
                  OutlinedButton(
                    onPressed: () {},
                    child: const Text('删除'),
                  )
                ],
              ),
              const SizedBox(width: 5.0),
              OutlinedButton(
                onPressed: () {},
                child: const Text('增加参数'),
              )
            ],
          ),
        ),
        const SizedBox(width: 5.0),
      ],
    );
  }

  Widget _buildAction() {
    return InkWell(
      onTap: () {},
      radius: 0,
      child: const Text('回测'),
    );
  }

  Widget _buildConfig(Color dividerColor) {
    return Column(
      children: [
        _buildParam(dividerColor),
        Divider(thickness: 1, height: 1, color: dividerColor),
        _buildAction(),
        Divider(thickness: 1, height: 1, color: dividerColor),
      ],
    );
    // return Text('config: ${widget.node.text}');
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

  @override
  bool get wantKeepAlive => true;
}
