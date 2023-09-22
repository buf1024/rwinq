import 'package:flutter/material.dart';
import 'package:winq/src/app/nav.dart';

import '../app/iconfont.dart';

const double kStatusBarHeight = 28.0;

class StatusBar extends StatefulWidget {
  final void Function(NavType type) onStatusTap;
  const StatusBar({super.key, required this.onStatusTap});

  @override
  State<StatusBar> createState() => _StatusBarState();
}

class _StatusBarState extends State<StatusBar> {
  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;

    return SizedBox(
      height: kStatusBarHeight,
      child: Column(
        children: [
          Divider(
            height: 1,
            thickness: 1,
            color: dividerColor,
          ),
          Padding(
            padding: const EdgeInsets.only(
                left: 25.0, top: 2.0, bottom: 2.0, right: 15.0),
            child: Row(
              children: [
                Tooltip(
                  message: '最新行情时间\n abc',
                  decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(5.0)),
                  textStyle:
                      const TextStyle(color: Colors.white, fontSize: 10.0),
                  verticalOffset: 10.0,
                  child: Text(
                    '11:24:30',
                    style: TextStyle(
                        fontSize: 13.0, color: Colors.white.withOpacity(0.8)),
                  ),
                ),

                const SizedBox(
                  width: 5.0,
                ),
                // TODO 根据配置滚动显示， 最多显示两个
                Tooltip(
                  message: '大盘行情',
                  decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(5.0)),
                  textStyle:
                      const TextStyle(color: Colors.white, fontSize: 10.0),
                  verticalOffset: 10.0,
                  child: Text(
                    '上证(33305.34 +0.03%)',
                    style: TextStyle(
                        fontSize: 13.0, color: Colors.white.withOpacity(0.8)),
                  ),
                ),
                const SizedBox(
                  width: 5.0,
                ),
                Tooltip(
                  message: '大盘行情',
                  decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(5.0)),
                  textStyle:
                      const TextStyle(color: Colors.white, fontSize: 10.0),
                  verticalOffset: 10.0,
                  child: Text(
                    '深证(33305.34 +0.03%)',
                    style: TextStyle(
                        fontSize: 13.0, color: Colors.white.withOpacity(0.8)),
                  ),
                ),
                const SizedBox(
                  width: 5.0,
                ),
                const Spacer(),
                // 跳转对应模块
                Tooltip(
                  message: '数据未完全同步\n\n最新时间: 2023-02-12',
                  decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(5.0)),
                  textStyle:
                      const TextStyle(color: Colors.white, fontSize: 10.0),
                  verticalOffset: 10.0,
                  child: GestureDetector(
                    onTap: () => widget.onStatusTap.call(NavType.data),
                    child: Text(
                      '数据(⚠️)',
                      style: TextStyle(
                          fontSize: 13.0, color: Colors.white.withOpacity(0.8)),
                    ),
                  ),
                ),
                const SizedBox(
                  width: 5.0,
                ),
                Tooltip(
                  message: '策略回测: 7',
                  decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(5.0)),
                  textStyle:
                      const TextStyle(color: Colors.white, fontSize: 10.0),
                  verticalOffset: 10.0,
                  child: GestureDetector(
                    onTap: () => widget.onStatusTap.call(NavType.strategy),
                    child: Text(
                      '回测(7)',
                      style: TextStyle(
                          fontSize: 13.0, color: Colors.white.withOpacity(0.8)),
                    ),
                  ),
                ),
                const SizedBox(
                  width: 5.0,
                ),
                Tooltip(
                  message: '算法交易: 2',
                  decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(5.0)),
                  textStyle:
                      const TextStyle(color: Colors.white, fontSize: 10.0),
                  verticalOffset: 10.0,
                  child: GestureDetector(
                    onTap: () => widget.onStatusTap.call(NavType.trade),
                    child: Text(
                      '交易(2)',
                      style: TextStyle(
                          fontSize: 13.0, color: Colors.white.withOpacity(0.8)),
                    ),
                  ),
                ),
                const SizedBox(
                  width: 5.0,
                ),
                // 有通知则闪烁
                Tooltip(
                  message: '通知',
                  decoration: BoxDecoration(
                      color: Colors.black.withOpacity(0.2),
                      borderRadius: BorderRadius.circular(5.0)),
                  textStyle:
                      const TextStyle(color: Colors.white, fontSize: 10.0),
                  verticalOffset: 10.0,
                  child: GestureDetector(
                      onTap: () =>
                          widget.onStatusTap.call(NavType.notification),
                      child: const Icon(
                        IconFont.notification,
                        size: 15,
                      )),
                ),
              ],
            ),
          )
        ],
      ),
    );
  }
}
