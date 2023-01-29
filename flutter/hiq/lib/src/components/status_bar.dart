import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';

class StatusBar extends StatefulWidget {
  final Widget? child;
  const StatusBar({super.key, this.child});

  @override
  State<StatusBar> createState() => _StatusBarState();
}

class _StatusBarState extends State<StatusBar> {
  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;

    return SizedBox(
      height: kWindowCaptionHeight,
      child: Column(
        children: [
          Divider(
            height: 1,
            thickness: 1,
            color: dividerColor,
          ),
          widget.child ?? Container()
        ],
      ),
    );
  }
}
