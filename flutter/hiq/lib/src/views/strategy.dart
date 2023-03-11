import 'package:flutter/material.dart';
import 'package:flutter/src/widgets/container.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:hiq/src/components/tab_title.dart';

class StrategyView extends StatefulWidget {
  const StrategyView({super.key});

  @override
  State<StrategyView> createState() => _StrategyViewState();
}

class _StrategyViewState extends State<StrategyView>
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
    return Row(children: [
      _buildSelection(),
      Expanded(
          child: Row(
        children: [
          _buildResizeDiv(dividerColor),
          Expanded(
              child: Column(
            children: [
              _buildTabTitle(),
              Divider(
                thickness: 1,
                height: 1,
                color: dividerColor,
              ),
              Expanded(child: const Text('content'))
            ],
          ))
        ],
      ))
    ]);
  }

  Widget _buildTabTitle() {
    return Row(children: [
      GestureDetector(
          onTap: () {
            setState(() {
              isHideInfo = !isHideInfo;
            });
          },
          child: Container(
            padding: const EdgeInsets.symmetric(vertical: 5.0, horizontal: 5.0),
            child: Tooltip(
              message: isHideInfo ? '显示详细信息' : '隐藏详细信息',
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
          )),
      Expanded(
          child: SizedBox(
              height: 30,
              child: ListView(
                  scrollDirection: Axis.horizontal,
                  children: tabTitle.asMap().entries.map((e) {
                    return TabTitleWidget(title: '测试', isActive: true);
                  }).toList())))
    ]);
  }

  Widget _buildSelection() {
    return Container(
      width: 150.0,
      child: Column(
        children: [
          Row(
            children: [
              Icon(Icons.star_outline),
              Icon(Icons.add_outlined),
              Icon(Icons.remove_outlined),
              const Spacer(),
              Icon(Icons.search_outlined),
            ],
          ),
          Expanded(child: const Text('策略页面'))
        ],
      ),
    );
  }

  Widget _buildResizeDiv(Color dividerColor) {
    double canvasWidth = MediaQuery.of(context).size.width;
    return MouseRegion(
      cursor: SystemMouseCursors.resizeColumn,
      child: GestureDetector(
        onPanUpdate: (details) {
          // double newWidth = rightWidth - details.delta.dx;
          // if (newWidth > kMinRightWidth &&
          //     newWidth + kMinLeftWidth <= canvasWidth) {
          //   setState(() {
          //     rightWidth = newWidth;
          //   });
          // }
        },
        child: VerticalDivider(
          thickness: 2,
          width: 2,
          color: dividerColor,
        ),
      ),
    );
  }

  @override
  bool get wantKeepAlive => true;
}
