import 'package:flutter/material.dart';

class ResearchView extends StatefulWidget {
  const ResearchView({super.key});

  @override
  State<ResearchView> createState() => _ResearchViewState();
}

class _ResearchViewState extends State<ResearchView> with AutomaticKeepAliveClientMixin{
  @override
  Widget build(BuildContext context) {
    return Container(child: Text('投研页面'));
  }
  
  @override
  bool get wantKeepAlive => true;
}
