import 'package:flutter/material.dart';
import 'package:hiq/src/views/strategy/backtest_layout.dart';

class BacktestObject {
  List<String> tabTitle = [];
  List<Widget> tabWidget = [];
  List<BacktestLayout> tabWidgetLayout = [];
  int tabIndex = 0;
  BacktestObject(
      {required this.tabIndex,
      required this.tabWidget,
      required this.tabTitle,
      required this.tabWidgetLayout});
}