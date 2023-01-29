import 'package:flutter/material.dart';
import 'package:flutter/src/widgets/container.dart';
import 'package:flutter/src/widgets/framework.dart';

class FavoriteView extends StatefulWidget {
  const FavoriteView({super.key});

  @override
  State<FavoriteView> createState() => _FavoriteViewState();
}

class _FavoriteViewState extends State<FavoriteView> {
  @override
  Widget build(BuildContext context) {
    return Container(
      child: SingleChildScrollView(
        child: DataTable(
          border: TableBorder.all(),
          columns: [
            DataColumn(label: Text('id')),
            DataColumn(label: Text('code')),
            DataColumn(label: Text('name')),
          ],
          rows: [
            ...List.generate(100, (v) => v).map(
              (e) {
                return DataRow(
                  cells: [
                    DataCell(Text('#$e')),
                    DataCell(Text('#$e')),
                    DataCell(Text('#$e')),
                  ],
                );
              },
            ).toList()
          ],
        ),
      ),
    );
  }
}
