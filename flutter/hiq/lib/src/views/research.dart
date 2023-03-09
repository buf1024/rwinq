import 'package:flutter/material.dart';

class ResearchView extends StatefulWidget {
  const ResearchView({super.key});

  @override
  State<ResearchView> createState() => _ResearchViewState();
}

class _ResearchViewState extends State<ResearchView> {
  @override
  Widget build(BuildContext context) {
    return Container(child: Text('投研页面'));
  }
}
