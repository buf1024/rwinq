import 'package:flutter/material.dart';

class DataSyncView extends StatefulWidget {
  const DataSyncView({super.key});

  @override
  State<DataSyncView> createState() => _DataSyncViewState();
}

class _DataSyncViewState extends State<DataSyncView>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;
  @override
  void initState() {
    super.initState();

    _tabController = TabController(vsync: this, length: 3);
  }

  @override
  void dispose() {
    super.dispose();
    _tabController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
        child: SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          _paddingWidget(_buildOptions()),
          _paddingWidget(_buildSync()),
          _paddingWidget(_buildAction())
        ],
      ),
    ));
  }

  Widget _buildAction() {
    return Row(children: [
      const Divider(
          height: 2,
          thickness: 2,
        ),
        Spacer(),
      ElevatedButton(onPressed: () {}, child: Text('停止'),),
      SizedBox(width: 10,),
      ElevatedButton(onPressed: () {}, child: Text('全量同步'),)
    ],);
  }

  Widget _paddingWidget(Widget child) {
    return Padding(
      padding: const EdgeInsets.only(top: 20, left: 40, right: 40),
      child: child,
    );
  }

  Widget _buildGroupHeader(String name) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.start,
      children: [
        Padding(
          padding: const EdgeInsets.all(10.0),
          child: Text(
            name,
            style: const TextStyle(fontWeight: FontWeight.bold, fontSize: 15.0),
          ),
        ),
        const Divider(
          height: 2,
          thickness: 2,
        ),
      ],
    );
  }

  Widget _buildOptions() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _buildGroupHeader('基本选项'),
        const SizedBox(
          height: 10.0,
        ),
        Row(
          children: [
            const SizedBox(
              width: 80,
              child: Align(
                alignment: Alignment.centerRight,
                child: Text('并发数:'),
              ),
            ),
            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 10),
              child: SizedBox(
                width: 120,
                child: Tooltip(
                  message: '并发数过高可能导致被封',
                  child: TextField(
                    keyboardType: TextInputType.numberWithOptions(),
                    decoration: InputDecoration(
                        border: OutlineInputBorder(), hintText: '并发数'),
                  ),
                ),
              ),
            ),
            const SizedBox(
              width: 80,
              child: Align(
                alignment: Alignment.centerRight,
                child: Text('切分份数:'),
              ),
            ),
            const Padding(
              padding: EdgeInsets.symmetric(horizontal: 10),
              child: SizedBox(
                width: 120,
                child: TextField(
                  decoration: InputDecoration(
                      border: OutlineInputBorder(), hintText: '切分份数'),
                ),
              ),
            ),
            const SizedBox(
              width: 100,
              child: Align(
                alignment: Alignment.centerRight,
                child: Text('忽略基础数据:'),
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 10),
              child: Checkbox(
                splashRadius: 4.0,
                value: true,
                onChanged: (value) {},
              ),
            )
          ],
        ),
        const SizedBox(
          height: 10.0,
        ),
        Row(
          children: [
            const SizedBox(
              width: 80,
              child: Align(
                alignment: Alignment.centerRight,
                child: Text('定时同步:'),
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 10),
              child: Checkbox(
                splashRadius: 4.0,
                value: true,
                onChanged: (value) {
                  showDialog(
                      context: context,
                      builder: (context) {
                        return TimePickerDialog(
                            initialTime: TimeOfDay(hour: 15, minute: 30));
                      });
                },
              ),
            ),
            Text('15:30')
          ],
        )
      ],
    );
  }

  Widget _buildSync() {
    return Stack(
      children: const [
        AbsorbPointer(
          absorbing: false,
          child: TabWidget(),
        ),
        SizedBox(
          height: 500,
          child: Align(
            alignment: Alignment.center,
            child: CircularProgressIndicator(),
          ),
        )
      ],
    );
  }
}

class TabWidget extends StatefulWidget {
  const TabWidget({super.key});

  @override
  State<TabWidget> createState() => _TabWidgetState();
}

class _TabWidgetState extends State<TabWidget>
    with SingleTickerProviderStateMixin {
  late TabController _tabController;

  List<String> items = ['可转债', '股票', '场内基金'];
  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: items.length, vsync: this);
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        TabBar(
            controller: _tabController,
            tabs: items.map((e) {
              return Text(
                e,
                style: const TextStyle(color: Colors.white30),
              );
            }).toList()),
        SizedBox(
          // width: MediaQuery.of(context).size.width,
          height: 500,
          child: TabBarView(
            physics: AlwaysScrollableScrollPhysics(),
            controller: _tabController,
            children: items.map((e) {
              // return Center(
              //   child: Text(
              //     'fuck: www$e ',
              //     style: const TextStyle(color: Colors.white30),
              //   ),
              // );
              return Wrap(
                children: const [
                  Padding(
                    padding: EdgeInsets.all(10.0),
                    child: SyncCardWidget(
                      syncItemText: '可转债基本信息',
                      syncItemName: 'fund_info',
                      latestDate: '2022-12-31',
                      isSyncing: false,
                    ),
                  ),
                  Padding(
                    padding: EdgeInsets.all(10.0),
                    child: SyncCardWidget(
                      syncItemText: '可转债日线',
                      syncItemName: 'fund_daily',
                      latestDate: '2022-12-31',
                      isSyncing: true,
                    ),
                  ),
                  Padding(
                    padding: EdgeInsets.all(10.0),
                    child: SyncCardWidget(
                      syncItemText: '可转债日线',
                      syncItemName: 'fund_daily',
                      latestDate: '2022-12-31',
                      isSyncing: true,
                    ),
                  ),
                  Padding(
                    padding: EdgeInsets.all(10.0),
                    child: SyncCardWidget(
                      syncItemText: '可转债日线',
                      syncItemName: 'fund_daily',
                      latestDate: '2022-12-31',
                      isSyncing: true,
                    ),
                  ),
                  Padding(
                    padding: EdgeInsets.all(10.0),
                    child: SyncCardWidget(
                      syncItemText: '可转债日线',
                      syncItemName: 'fund_daily',
                      latestDate: '2022-12-31',
                      isSyncing: true,
                    ),
                  ),
                  Padding(
                    padding: EdgeInsets.all(10.0),
                    child: SyncCardWidget(
                      syncItemText: '可转债日线',
                      syncItemName: 'fund_daily',
                      latestDate: '2022-12-31',
                      isSyncing: true,
                    ),
                  ),
                  Padding(
                    padding: EdgeInsets.all(10.0),
                    child: SyncCardWidget(
                      syncItemText: '可转债日线',
                      syncItemName: 'fund_daily',
                      latestDate: '2022-12-31',
                      isSyncing: true,
                    ),
                  ),
                  Padding(
                    padding: EdgeInsets.all(10.0),
                    child: SyncCardWidget(
                      syncItemText: '可转债日线',
                      syncItemName: 'fund_daily',
                      latestDate: '2022-12-31',
                      isSyncing: true,
                    ),
                  ),
                ],
              );
            }).toList(),
          ),
        ),
      ],
    );
  }
}

class SyncCardWidget extends StatelessWidget {
  final String syncItemText;
  final String syncItemName;
  final String latestDate;
  final bool isSyncing;
  const SyncCardWidget(
      {super.key,
      required this.syncItemText,
      required this.syncItemName,
      required this.latestDate,
      required this.isSyncing});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 200,
      // height: 60,
      decoration: BoxDecoration(
          color: Colors.white70,
          border: Border.all(),
          borderRadius: const BorderRadius.all(Radius.circular(8.0))),
      child: Column(
        children: [
          Padding(
            padding: EdgeInsets.all(8.0),
            child: Text('$syncItemText($syncItemName)\n同步时间: $latestDate'),
          ),
          Padding(
              padding: EdgeInsets.all(8.0),
              child: Row(
                children: [
                  Expanded(
                      child: ElevatedButton(
                    child: isSyncing
                        ? const SizedBox(
                            height: 15,
                            width: 15,
                            child: CircularProgressIndicator(
                              strokeWidth: 2,
                              backgroundColor: Colors.white,
                            ),
                          )
                        : Text('同步'),
                    onPressed: () {},
                  ))
                ],
              ))
        ],
      ),
    );
  }
}
