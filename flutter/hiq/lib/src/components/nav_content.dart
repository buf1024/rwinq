import 'package:flutter/material.dart';

const kMinNavContentWidth = 60.0;

class NavContent extends StatefulWidget {
  final Widget child;

  const NavContent({super.key, required this.child});

  @override
  State<NavContent> createState() => _NavContentState();
}

class _NavContentState extends State<NavContent> {
  double width = 128.0;
  @override
  Widget build(BuildContext context) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SingleChildScrollView(
          child: SizedBox(
            width: width,
            // decoration: BoxDecoration(border: Border.all()),
            child: SingleChildScrollView(
              scrollDirection: Axis.horizontal,
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  widget.child,
                ],
              ),
            ),
            
          ),
        ),
        MouseRegion(
          cursor: SystemMouseCursors.resizeColumn,
          child: GestureDetector(
            onPanUpdate: (details) {
              setState(() {
                width += details.delta.dx;
                if (width <= kMinNavContentWidth) {
                  width = kMinNavContentWidth;
                }
              });
            },
            child: VerticalDivider(
              width: 2,
              thickness: 2,
              color: Theme.of(context).dividerColor,
            ),
          ),
        )
      ],
    );
  }
}
