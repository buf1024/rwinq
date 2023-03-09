import 'package:flutter/material.dart';

enum NavType {
  dashboard,
  data,
  strategy,
  research,
  trade,
  favorite,
  notification
}

enum NavPos { top, bottom }

class NavItem {
  NavType type;
  NavPos pos;
  String label;
  IconData iconData;

  NavItem(
      {required this.type,
      required this.pos,
      required this.label,
      required this.iconData});
}
