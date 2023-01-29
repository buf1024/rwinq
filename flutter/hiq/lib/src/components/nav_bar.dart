import 'package:flutter/material.dart';


enum NavType { data, strategy, analyze, quotation, trade, favorite, config }

enum NavPos { top, bottom }

class NavItem {
  NavType type;
  NavPos pos;
  String tooltip;
  IconData iconData;

  NavItem(
      {required this.type,
      required this.pos,
      required this.tooltip,
      required this.iconData});
}

class NavBar extends StatefulWidget {
  final List<NavItem> topNavTabs;
  final List<NavItem> bottomNavTabs;
  final NavType actType;
  final ValueChanged<NavItem>? onTap;
  const NavBar(
      {super.key,
      required this.topNavTabs,
      required this.bottomNavTabs,
      required this.actType,
      this.onTap});

  @override
  State<NavBar> createState() => _NavBarState();
}

class _NavBarState extends State<NavBar> {
  late NavType actType;

  Set<NavType> mouseInSet = {};

  @override
  void initState() {
    super.initState();
    actType = widget.actType;
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ..._buildWidget(widget.topNavTabs, Colors.red, Colors.blue),
        const Spacer(),
        ..._buildWidget(widget.bottomNavTabs, Colors.red, Colors.blue),
      ],
    );
  }

  List<Widget> _buildWidget(
      List<NavItem> navTabs, Color actColor, Color inActColor) {
    return navTabs.map((elem) {
      return Padding(
        padding: const EdgeInsets.symmetric(vertical: 15.0),
        child: Tooltip(
          message: elem.tooltip,
          child: MouseRegion(
            onEnter: (event) {
              setState(() {
                mouseInSet.add(elem.type);
              });
            },
            onExit: (event) {
              setState(() {
                mouseInSet.remove(elem.type);
              });
            },
            child: GestureDetector(
              onTap: (() {
                setState(() {
                  actType = elem.type;
                });
                widget.onTap?.call(elem);
              }),
              child: Icon(elem.iconData,
                  color: actType == elem.type
                      ? actColor
                      : (mouseInSet.contains(elem.type)
                          ? actColor
                          : inActColor)),
            ),
          ),
        ),
      );
    }).toList();
  }
}
