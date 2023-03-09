import 'package:flutter/material.dart';

class DataSyncView extends StatefulWidget {
  const DataSyncView({super.key});

  @override
  State<DataSyncView> createState() => _DataSyncViewState();
}

class _DataSyncViewState extends State<DataSyncView>
    with SingleTickerProviderStateMixin {
  List<String> items = ['股票', '场内基金', '可转债'];
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
        child: Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        _buildAction(),
        const SizedBox(
          height: 15.0,
        ),
        _buildContent(),
      ],
    ));
  }

  Widget _buildAction() {
    return Row(
      children: [
        ElevatedButton(
          onPressed: () {},
          style: ElevatedButton.styleFrom(
              backgroundColor: Colors.red.withOpacity(0.8)),
          child: const Text(
            '全部停止',
            style: TextStyle(fontSize: 12.0),
          ),
        ),
        const SizedBox(
          width: 10,
        ),
        ElevatedButton(
          style: ElevatedButton.styleFrom(
              backgroundColor: Colors.green.withOpacity(0.8)),
          onPressed: () {},
          child: const Text('全部开始', style: TextStyle(fontSize: 12.0)),
        ),
        const Spacer()
      ],
    );
  }

  Widget _buildContent() {
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
          height: 650.0,
          child: TabBarView(
            physics: const AlwaysScrollableScrollPhysics(),
            controller: _tabController,
            children: items.map((e) {
              if (e == '股票') {
                return _buildStockWidget();
              } else if (e == '场内基金') {
                return _buildFundWidget();
              } else if (e == '可转债') {
                return _buildBondWidget();
              }
              return Container();
            }).toList(),
          ),
        ),
      ],
    );
  }

  Widget _buildFundWidget() {
    return Wrap(
      children: const [
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'fund_daily',
            syncName: '日线',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'fund_net',
            syncName: '净值',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 110,
            isSyncing: true,
          ),
        ),
      ],
    );
  }

  Widget _buildBondWidget() {
    return Wrap(
      children: const [
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'bond_daily',
            syncName: '日线',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
      ],
    );
  }

  Widget _buildStockWidget() {
    return Wrap(
      children: const [
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'index_daily',
            syncName: '指数日线',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_daily',
            syncName: '股票日线',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_index',
            syncName: '股票指标',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_industry',
            syncName: '行业信息',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_industry_daily',
            syncName: '行业日线',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_industry_detail',
            syncName: '行业明细',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_concept',
            syncName: '概念信息',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_concept_daily',
            syncName: '概念日线',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_concept_detail',
            syncName: '概念明细',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_yjbb',
            syncName: '业绩报表',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
        Padding(
          padding: EdgeInsets.all(10.0),
          child: SyncCardWidget(
            syncTab: 'stock_margin',
            syncName: '融资融券',
            recordDate: '2022-12-31',
            syncDate: '2022-12-31',
            recordNum: 1100,
            isSyncing: false,
          ),
        ),
      ],
    );
  }
}

class SyncCardWidget extends StatelessWidget {
  final String syncTab;
  final String syncName;
  final int recordNum;
  final String recordDate;
  final String syncDate;
  final bool isSyncing;
  const SyncCardWidget(
      {super.key,
      required this.syncTab,
      required this.syncName,
      required this.recordNum,
      required this.recordDate,
      required this.syncDate,
      required this.isSyncing});

  @override
  Widget build(BuildContext context) {
    Color dividerColor = Theme.of(context).dividerColor;
    return Container(
      width: 240.0,
      // height: 60,
      decoration: BoxDecoration(
          color: Colors.grey.withOpacity(0.8),
          border: Border.all(),
          borderRadius: const BorderRadius.all(Radius.circular(8.0))),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
              padding: const EdgeInsets.all(5.0),
              child: Text(
                '$syncName($syncTab)',
                style: const TextStyle(fontSize: 12.0),
              )),
          Divider(
            height: 1,
            thickness: 1,
            color: dividerColor.withOpacity(0.5),
          ),
          Container(
            padding: const EdgeInsets.all(4.0),
            child: Text(
              '已同步记录数: $recordNum',
              style: const TextStyle(
                fontSize: 12.0,
              ),
            ),
          ),
          Container(
            padding: const EdgeInsets.all(4.0),
            child: Text(
              '已同步最新日期: $recordDate',
              style: const TextStyle(
                fontSize: 12.0,
              ),
            ),
          ),
          Container(
            padding: const EdgeInsets.all(4.0),
            child: Text(
              '最近同步时间: $syncDate',
              style: const TextStyle(
                fontSize: 12.0,
              ),
            ),
          ),
          Divider(
            height: 1,
            thickness: 1,
            color: dividerColor.withOpacity(0.5),
          ),
          Padding(
              padding: const EdgeInsets.all(5.0),
              child: Row(
                children: [
                  const Spacer(),
                  ..._syncActions(),
                  const SizedBox(
                    width: 5.0,
                  ),
                  GestureDetector(
                    onTap: () {},
                    child: Container(
                      padding: const EdgeInsets.only(
                          top: 2.0, bottom: 2.0, left: 4.0, right: 5.0),
                      decoration: BoxDecoration(
                        color: Colors.yellow.withOpacity(0.8),
                        borderRadius: BorderRadius.circular(2.0),
                      ),
                      child: const Text(
                        '日志',
                        style: TextStyle(
                          fontSize: 12.0,
                        ),
                      ),
                    ),
                  )
                ],
              )),
        ],
      ),
    );
  }

  List<Widget> _syncActions() {
    List<Widget> widgets = [];
    if (isSyncing) {
      widgets.add(const SizedBox(
        height: 15,
        width: 15,
        child: CircularProgressIndicator(
          strokeWidth: 1,
          backgroundColor: Colors.white,
        ),
      ));
      widgets.add(const SizedBox(
        width: 5.0,
      ));
      widgets.add(GestureDetector(
        onTap: () {},
        child: Container(
          padding: const EdgeInsets.only(
              top: 2.0, bottom: 2.0, left: 4.0, right: 5.0),
          decoration: BoxDecoration(
            color: Colors.red.withOpacity(0.8),
            borderRadius: BorderRadius.circular(2.0),
          ),
          child: const Text(
            '停止',
            style: TextStyle(
              fontSize: 12.0,
            ),
          ),
        ),
      ));
    } else {
      widgets.add(GestureDetector(
        onTap: () {},
        child: Container(
          padding: const EdgeInsets.only(
              top: 2.0, bottom: 2.0, left: 4.0, right: 5.0),
          decoration: BoxDecoration(
            color: Colors.blue.withOpacity(0.8),
            borderRadius: BorderRadius.circular(2.0),
          ),
          child: const Text(
            '同步',
            style: TextStyle(
              fontSize: 12.0,
            ),
          ),
        ),
      ));
    }
    return widgets;
  }
}
