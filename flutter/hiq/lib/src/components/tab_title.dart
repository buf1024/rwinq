import 'package:flutter/material.dart';

class TabTitleWidget extends StatefulWidget {
  final ValueChanged<Offset>? onSecondaryTap;
  final VoidCallback? onTap;
  final VoidCallback? onCloseTap;
  final String title;
  final String? tooltip;
  final bool isActive;
  const TabTitleWidget(
      {super.key,
      this.onSecondaryTap,
      this.onCloseTap,
      required this.title,
      this.tooltip,
      required this.isActive, this.onTap});

  @override
  State<TabTitleWidget> createState() => _TabTitleWidgetState();
}

class _TabTitleWidgetState extends State<TabTitleWidget> {
  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onSecondaryTapDown: (details) {
        widget.onSecondaryTap?.call(details.globalPosition);
      },
      onTap: () {
        widget.onTap?.call();
      },
      child: Container(
        decoration: BoxDecoration(
          color: widget.isActive ? Colors.grey.withOpacity(0.2) : null,
          // border: Border.all(color: Colors.grey.withOpacity(0.15)),
        ),
        padding: const EdgeInsets.symmetric(
          horizontal: 8.0,
        ),
        child: Row(
          children: [
            const Icon(
              Icons.description_outlined,
              size: 15.0,
            ),
            Text(
              widget.title,
              overflow: TextOverflow.ellipsis,
              style: TextStyle(
                  fontSize: 14.0, color: Colors.white.withOpacity(0.8)),
            ),
            GestureDetector(
              onTap: () {
                widget.onCloseTap?.call();
              },
              child: const Icon(
                Icons.close_outlined,
                size: 15.0,
              ),
            )
          ],
        ),
      ),
    );
  }
}
