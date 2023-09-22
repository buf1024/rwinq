import 'package:flutter/material.dart';
import 'package:winq/src/app/iconfont.dart';
import 'package:window_manager/window_manager.dart';
import 'dart:io';

const kTitleBarHeight = 32.0;

class TitleBar extends StatefulWidget {
  final Widget? child;
  final VoidCallback? onConfigCall;
  final VoidCallback? onLockCall;
  const TitleBar({super.key, this.child, this.onConfigCall, this.onLockCall});

  @override
  State<TitleBar> createState() => _TitleBarState();
}

class _TitleBarState extends State<TitleBar> {
  Map<ThemeMode, IconData> themeIconMap = {
    ThemeMode.dark: IconFont.darkMode,
    ThemeMode.light: IconFont.lightMode,
    ThemeMode.system: IconFont.systemMode,
  };
  Map<ThemeMode, String> themeLabelMap = {
    ThemeMode.dark: '深色模式',
    ThemeMode.light: '浅色模式',
    ThemeMode.system: '跟随系统',
  };
  late ThemeMode themeMode;
  @override
  void initState() {
    super.initState();
    themeMode = ThemeMode.dark;
  }

  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;
    return DragToMoveArea(
        child: Column(
      children: [
        SizedBox(
          height: kTitleBarHeight,
          child: Stack(
            children: [
              Center(child: widget.child ?? Container()),
              _funcButtons()
            ],
          ),
        ),
        Divider(
          height: 1,
          thickness: 1,
          color: dividerColor,
        )
      ],
    ));
  }

  Widget _windowsButtons() {
    Brightness brightness = Theme.of(context).brightness;
    return Row(
      children: [
        WindowCaptionButton.close(
          brightness: brightness,
          onPressed: () {
            windowManager.close();
          },
        ),
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
      ],
    );
  }

  Widget _funcButtons() {
    return Row(children: [
      Platform.isLinux || Platform.isLinux ? _windowsButtons() : Container(),
      const Spacer(),
      GestureDetector(
        onTap: () {
          widget.onLockCall?.call();
        },
        child: Tooltip(
          message: '锁屏',
          decoration: BoxDecoration(
              color: Colors.black.withOpacity(0.2),
              borderRadius: BorderRadius.circular(5.0)),
          textStyle: const TextStyle(color: Colors.white, fontSize: 10.0),
          verticalOffset: 10.0,
          child: const Icon(
            IconFont.lock,
            size: 16.0,
          ),
        ),
      ),
      const SizedBox(width: 10.0),
      GestureDetector(
        onTap: () {
          widget.onConfigCall?.call();
        },
        child: Tooltip(
          message: '系统配置',
          decoration: BoxDecoration(
              color: Colors.black.withOpacity(0.2),
              borderRadius: BorderRadius.circular(5.0)),
          textStyle: const TextStyle(color: Colors.white, fontSize: 10.0),
          verticalOffset: 10.0,
          child: const Icon(
            IconFont.config,
            size: 16.0,
          ),
        ),
      ),
      const SizedBox(width: 10.0),
      GestureDetector(
        onTap: () => showThemeSwitchDialog(),
        child: Tooltip(
          message: '外观模式: ${themeLabelMap[themeMode]}',
          decoration: BoxDecoration(
              color: Colors.black.withOpacity(0.2),
              borderRadius: BorderRadius.circular(5.0)),
          textStyle: const TextStyle(color: Colors.white, fontSize: 10.0),
          verticalOffset: 10.0,
          child: Icon(
            themeIconMap[themeMode],
            size: 16.0,
          ),
        ),
      ),
      const SizedBox(
        width: 20.0,
      )
    ]);
  }

  void showThemeSwitchDialog() {
    showDialog(
        context: context,
        barrierDismissible: true,
        barrierColor: Colors.black.withOpacity(0),
        builder: (BuildContext context) {
          return Dialog(
            alignment: const Alignment(1.1, -0.95),
            shadowColor: Colors.grey.withOpacity(0.8),
            backgroundColor: Theme.of(context).scaffoldBackgroundColor,
            elevation: 2.0,
            child: _ThemeSwitchWidget(
              themeIconMap: themeIconMap,
              themeLabelMap: themeLabelMap,
              themeMode: themeMode,
              onThemeSwitch: (mode) => onThemeSwitch(mode),
            ),
          );
        });
  }

  void onThemeSwitch(ThemeMode mode) {
    if (themeMode != mode) {
      setState(() {
        themeMode = mode;
      });
    }
  }
}

class _ThemeSwitchWidget extends StatefulWidget {
  final Map<ThemeMode, IconData> themeIconMap;
  final Map<ThemeMode, String> themeLabelMap;
  final ThemeMode themeMode;
  final ValueChanged<ThemeMode> onThemeSwitch;
  const _ThemeSwitchWidget(
      {required this.themeIconMap,
      required this.themeMode,
      required this.themeLabelMap,
      required this.onThemeSwitch});

  @override
  State<_ThemeSwitchWidget> createState() => _ThemeSwitchWidgetState();
}

class _ThemeSwitchWidgetState extends State<_ThemeSwitchWidget> {
  final Set<ThemeMode> themeMouseInSet = {};
  late ThemeMode themeMode;
  @override
  void initState() {
    super.initState();
    themeMode = widget.themeMode;
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 10.0,
      height: 120.0,
      child: Column(
        children: _buildItem(),
      ),
    );
  }

  List<Widget> _buildItem() {
    return widget.themeIconMap.entries.map((entry) {
      return Padding(
        padding: const EdgeInsets.all(1.0),
        child: MouseRegion(
          onEnter: (event) {
            setState(() {
              themeMouseInSet.add(entry.key);
            });
          },
          onExit: (event) {
            setState(() {
              themeMouseInSet.remove(entry.key);
            });
          },
          child: GestureDetector(
            onTap: () {
              Navigator.of(context).pop();
              widget.onThemeSwitch(entry.key);
            },
            child: Container(
              padding: const EdgeInsets.all(8.0),
              decoration: _themeBoxDecoration(entry.key),
              child: Row(
                children: [
                  Icon(
                    entry.value,
                    size: 16.0,
                  ),
                  const SizedBox(
                    width: 12.0,
                  ),
                  Text(
                    widget.themeLabelMap[entry.key]!,
                    style: TextStyle(
                        fontSize: 14.0,
                        color: entry.key == themeMode
                            ? Colors.blue.withOpacity(0.8)
                            : Colors.white.withOpacity(0.8)),
                  )
                ],
              ),
            ),
          ),
        ),
      );
    }).toList();
  }

  BoxDecoration? _themeBoxDecoration(ThemeMode mode) {
    if (themeMouseInSet.contains(mode)) {
      return BoxDecoration(
          color: Colors.grey.withOpacity(0.2),
          borderRadius: BorderRadius.circular(5.0));
    }
    return null;
  }
}
