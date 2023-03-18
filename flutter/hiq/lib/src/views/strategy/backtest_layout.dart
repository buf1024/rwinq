import 'package:equatable/equatable.dart';

enum LayoutShow { topLeft, both, bottomRight }

class BacktestLayout extends Equatable {
  final LayoutShow horizontal;
  final LayoutShow vertical;

  const BacktestLayout({required this.horizontal, required this.vertical});

  @override
  List<Object?> get props => [horizontal, vertical];
}
