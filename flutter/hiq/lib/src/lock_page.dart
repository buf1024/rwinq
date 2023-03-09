import 'package:flutter/material.dart';
import 'package:hiq/src/app/iconfont.dart';
import 'package:hiq/src/app/nav.dart';
import 'package:hiq/src/components/nav_bar.dart';
import 'package:hiq/src/components/status_bar.dart';
import 'package:hiq/src/components/title_bar.dart';
import 'package:hiq/src/views/config.dart';
import 'package:hiq/src/views/dashboard.dart';
import 'package:hiq/src/views/data.dart';
import 'package:hiq/src/views/favorite.dart';
import 'package:hiq/src/views/research.dart';
import 'package:hiq/src/views/strategy.dart';
import 'package:hiq/src/views/trade.dart';
import 'package:path/path.dart';
import 'package:window_manager/window_manager.dart';

class LockPage extends StatefulWidget {
  const LockPage({super.key});

  @override
  State<LockPage> createState() => _LockPageState();
}

class _LockPageState extends State<LockPage> {
  @override
  void initState() {
    super.initState();

  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // backgroundColor: backgroundColor,
      body: Center(
        child: Column(
          children: [
            Spacer(),
            const Text('you are lock'),
            ElevatedButton(onPressed: () {
              Navigator.of(context).pop();
              
            }, style: ElevatedButton.styleFrom(
              backgroundColor: Colors.blue.withOpacity(0.8),
              elevation: 2.0,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(50.0))
            ),
            child: const Text('pop'))
            ,Spacer()
          ],
        ),
      )
    );
  }

}
