import 'package:flutter/material.dart';
import 'package:hiq/src/app/nav.dart';

const kNaviBarWidth = 55.0;

class NavBar extends StatefulWidget {
  final List<NavItem>? topNavTabs;
  final List<NavItem>? bottomNavTabs;
  final NavType actType;
  final ValueChanged<NavItem>? onTap;
  const NavBar(
      {super.key,
      this.topNavTabs,
      this.bottomNavTabs,
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
  void didUpdateWidget(covariant NavBar oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.actType != widget.actType) {
      actType = widget.actType;
    }
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: kNaviBarWidth,
      child: Column(
        children: [
          ..._buildWidget(widget.topNavTabs),
          const Spacer(),
          ..._buildWidget(widget.bottomNavTabs),
        ],
      ),
    );
  }

  BoxDecoration? _boxDecoration(NavType type) {
    if (actType == type || mouseInSet.contains(type)) {
      return BoxDecoration(
          color: Colors.grey.withOpacity(0.2),
          borderRadius: BorderRadius.circular(5.0));
    }
    return null;
  }

  List<Widget> _buildWidget(List<NavItem>? navTabs) {
    if (navTabs == null) {
      return [];
    }
    return navTabs.map((elem) {
      return Padding(
        padding: const EdgeInsets.all(5.0),
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
            child: Container(
              width: 48.0,
              height: 48.0,
              decoration: _boxDecoration(elem.type),
              padding: const EdgeInsets.all(5.0),
              child: Column(
                children: [
                  Icon(
                    elem.iconData,
                    color: Colors.white.withOpacity(0.8),
                    weight: 100,
                    size: 20.0,
                  ),
                  const SizedBox(
                    height: 1.0,
                  ),
                  Text(
                    elem.label,
                    style: TextStyle(
                        fontSize: 12.0, color: Colors.white.withOpacity(0.8)),
                  )
                ],
              ),
            ),
          ),
        ),
      );
    }).toList();
  }
}
