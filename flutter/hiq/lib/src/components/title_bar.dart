import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';

class TitleBar extends StatefulWidget {
  final Widget? child;
  const TitleBar({super.key, this.child});

  @override
  State<TitleBar> createState() => _TitleBarState();
}

class _TitleBarState extends State<TitleBar> {
  @override
  @override
  Widget build(BuildContext context) {
    Brightness brightness = Theme.of(context).brightness;
    return DragToMoveArea(
      child: Stack(
        children: [
          Container(
            height: kWindowCaptionHeight,
            child: Center(
              child: widget.child ?? Container(),
            ),
          ),
          Column(
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  WindowCaptionButton.minimize(
                    brightness: brightness,
                    onPressed: () async {
                      bool isMinimized = await windowManager.isMinimized();
                      if (isMinimized) {
                        windowManager.restore();
                      } else {
                        windowManager.minimize();
                      }
                    },
                  ),
                  FutureBuilder<bool>(
                    future: windowManager.isMaximized(),
                    builder:
                        (BuildContext context, AsyncSnapshot<bool> snapshot) {
                      print('snap: ${snapshot.data}');
                      if (snapshot.data == true) {
                        return WindowCaptionButton.unmaximize(
                          brightness: brightness,
                          onPressed: () {
                            windowManager.unmaximize();
                            setState(() {
                            
                          });
                          },
                        );
                      }
                      return WindowCaptionButton.maximize(
                        brightness: brightness,
                        onPressed: () async {
                          await windowManager.maximize();
                          bool ismax = await windowManager.isMaximized();
                          print('ismax=${ismax}');
                          setState(() {
                            
                          });
                        },
                      );
                    },
                  ),
                  WindowCaptionButton.close(
                    brightness: brightness,
                    onPressed: () {
                      windowManager.close();
                    },
                  ),
                ],
              ),
            ],
          )
        ],
      ),
    );
  }
}
