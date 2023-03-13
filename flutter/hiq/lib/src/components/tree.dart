import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';

class TreeWidget extends StatefulWidget {
  final TreeNode root;

  ValueChanged<TreeNode>? onDeleted;
  ValueChanged<TreeNode>? onStared;
  ValueChanged<TreeNode>? onSelected;

  final bool readOnly;

  bool? filteredStar;
  String? filteredText;

  TreeWidget(
      {super.key,
      required this.root,
      this.onDeleted,
      this.onStared,
      this.onSelected,
      required this.readOnly,
      this.filteredStar,
      this.filteredText});

  @override
  State<TreeWidget> createState() => _TreeWidgetState();
}

class _TreeWidgetState extends State<TreeWidget> {
  late TreeNode root;
  final double width = 1024.0;

  TreeNode? selectedNode;

  _TreeWidgetState();
  @override
  void initState() {
    super.initState();
    root = widget.root;
  }

  @override
  void didUpdateWidget(covariant TreeWidget oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.root != widget.root) {
      root = widget.root;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      width: width,
      padding: const EdgeInsets.all(2.0),
      child: SingleChildScrollView(
        scrollDirection: Axis.vertical,
        child: SingleChildScrollView(
          scrollDirection: Axis.horizontal,
          child: _buildTreeNode(root, 0),
        ),
      ),
    );
  }

  void _onEnter(TreeNode node) {
    setState(() {
      node.hover = true;
    });
  }

  void _onLeave(TreeNode node) {
    setState(() {
      node.hover = false;
    });
  }

  BoxDecoration? _boxDecoration(TreeNode node) {
    if (node.selected == null || node.hover == null) {
      return null;
    }
    if (node.selected! || node.hover!) {
      return BoxDecoration(
        color: Colors.grey.withOpacity(0.2),
      );
    }
    return null;
  }

  void _onSelected(TreeNode node, bool tryExpand) {
    if (selectedNode != null) {
      selectedNode!.selected = false;
    }

    selectedNode = node;
    selectedNode!.selected = true;
    if (!selectedNode!.isLeaf && tryExpand) {
      if (selectedNode!.expand == null) {
        selectedNode!.expand = true;
      } else {
        selectedNode!.expand = !selectedNode!.expand!;
      }
    }

    setState(() {});

    widget.onSelected?.call(node);
  }

  void _onExpandAll(bool expand) {
    _expandAll(root, expand);
    setState(() {});
  }

  void _expandAll(TreeNode node, bool expand) {
    if (!node.isLeaf) {
      node.expand = expand;
      if (!node.isEmpty) {
        for (var element in node.children!) {
          _expandAll(element, expand);
        }
      }
    }
  }

  void _onDelete(TreeNode node) {
    if (node != root) {
      _delete(root, node);
      setState(() {});
      widget.onDeleted?.call(node);
    }
  }

  bool _delete(TreeNode parent, TreeNode node) {
    if (!parent.isEmpty) {
      bool found = parent.children!.remove(node);
      if (!found) {
        for (var child in parent.children!) {
          if (_delete(child, node)) {
            return true;
          }
        }
      } else {
        return true;
      }
    }
    return false;
  }

  void _onStar(TreeNode node) {
    if (node.isLeaf) {
      if (node.star == null || !node.star!) {
        node.star = true;
      } else {
        node.star = false;
      }
      setState(() {});

      widget.onStared?.call(node);
    }
  }

  void _onShowMenu(TreeNode node, Offset offset) {
    _onSelected(node, false);
    showMenu(
        context: context,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(5.0),
        ),
        // surfaceTintColor: Colors.grey.withOpacity(0.8),

        position: RelativeRect.fromLTRB(
            offset.dx, offset.dy, offset.dx + 100, offset.dy + 100),
        items: [
          PopupMenuItem<Never>(
            mouseCursor: SystemMouseCursors.basic,
            height: 20.0,
            onTap: () => _onExpandAll(true),
            child: Container(
              padding: const EdgeInsets.all(8.0),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.start,
                children: [
                  const Icon(
                    Icons.unfold_more_double_outlined,
                    size: 18.0,
                  ),
                  const SizedBox(width: 8.0),
                  Text(
                    '全部展开',
                    style: TextStyle(
                        color: Colors.white.withOpacity(0.8), fontSize: 14.0),
                  )
                ],
              ),
            ),
          ),
          PopupMenuItem<Never>(
            mouseCursor: SystemMouseCursors.basic,
            height: 20.0,
            onTap: () => _onExpandAll(false),
            child: Container(
              padding: const EdgeInsets.all(8.0),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.start,
                children: [
                  const Icon(
                    Icons.unfold_less_double_outlined,
                    size: 18.0,
                  ),
                  const SizedBox(width: 8.0),
                  Text(
                    '全部折叠',
                    style: TextStyle(
                        color: Colors.white.withOpacity(0.8), fontSize: 14.0),
                  )
                ],
              ),
            ),
          ),
          ..._buildReadOnly(node),
        ],
        elevation: 8.0);
  }

  List<PopupMenuEntry<Never>> _buildReadOnly(TreeNode node) {
    if (widget.readOnly) {
      return [];
    }
    return [
      const PopupMenuDivider(),
      PopupMenuItem<Never>(
        mouseCursor: SystemMouseCursors.basic,
        height: 20.0,
        onTap: () => _onDelete(node),
        enabled: node != root,
        child: Container(
          padding: const EdgeInsets.all(8.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              const Icon(
                Icons.delete_outline_outlined,
                size: 18.0,
              ),
              const SizedBox(width: 8.0),
              Text(
                '删除',
                style: TextStyle(
                    color: Colors.white.withOpacity(0.8), fontSize: 14.0),
              )
            ],
          ),
        ),
      ),
      const PopupMenuDivider(),
      PopupMenuItem<Never>(
        mouseCursor: SystemMouseCursors.basic,
        height: 20.0,
        onTap: () => _onStar(node),
        enabled: node.isLeaf,
        child: Container(
          padding: const EdgeInsets.all(8.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              const Icon(
                Icons.star_border_outlined,
                size: 18.0,
              ),
              const SizedBox(width: 8.0),
              Text(
                node.star! ? '取消收藏' : '收藏',
                style: TextStyle(
                    color: Colors.white.withOpacity(0.8), fontSize: 14.0),
              )
            ],
          ),
        ),
      )
    ];
  }

  Widget _buildTreeNode(TreeNode node, int level) {
    if (node.isLeaf) {
      if (widget.filteredStar != null &&
          widget.filteredStar! &&
          (node.star == null || !node.star!)) {
        return Container();
      }
      if (widget.filteredText != null &&
          widget.filteredText!.isNotEmpty &&
          !node.text.contains(widget.filteredText!)) {
        return Container();
      }
    }
    return node.isLeaf || (!node.isLeaf && !node.isExpand)
        ? MouseRegion(
            onEnter: (event) => _onEnter(node),
            onExit: (event) => _onLeave(node),
            child: GestureDetector(
              onTap: () => _onSelected(node, true),
              onSecondaryTapDown: (details) =>
                  _onShowMenu(node, details.globalPosition),
              child: Container(
                width: width,
                decoration: _boxDecoration(node),
                child: Row(
                  children: [
                    ...List.generate(level, (index) {
                      return Padding(
                        padding: const EdgeInsets.only(left: 10.0),
                        child: Container(
                          width: 15.0,
                          height: 25.0,
                          decoration: BoxDecoration(
                            border: Border(
                                left: BorderSide(
                              color: Colors.grey.withOpacity(0.9),
                            )),
                          ),
                        ),
                      );
                    }),
                    Container(
                      padding: const EdgeInsets.symmetric(horizontal: 2.0),
                      child: node.isLeaf
                          ? Row(
                              children: [
                                const Icon(
                                  Icons.description_outlined,
                                  size: 16.0,
                                ),
                                const SizedBox(
                                  width: 4.0,
                                ),
                                Text(
                                  node.text,
                                  overflow: TextOverflow.ellipsis,
                                  style: TextStyle(
                                    fontSize: 14.0,
                                    color: Colors.white.withOpacity(0.8),
                                  ),
                                ),
                                const SizedBox(
                                  width: 4.0,
                                ),
                                node.star!
                                    ? const Icon(
                                        Icons.star_border_outlined,
                                        size: 16.0,
                                      )
                                    : Container(),
                              ],
                            )
                          : Row(
                              children: [
                                node.isEmpty
                                    ? Container()
                                    : const Icon(
                                        Icons.chevron_right_outlined,
                                        size: 18.0,
                                      ),
                                const SizedBox(
                                  width: 4.0,
                                ),
                                const Icon(
                                  Icons.folder_outlined,
                                  size: 16.0,
                                ),
                                const SizedBox(
                                  width: 4.0,
                                ),
                                Text(
                                  node.text,
                                  overflow: TextOverflow.ellipsis,
                                  style: TextStyle(
                                    fontSize: 14.0,
                                    color: Colors.white.withOpacity(0.8),
                                  ),
                                )
                              ],
                            ),
                    )
                  ],
                ),
              ),
            ),
          )
        : Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              MouseRegion(
                onEnter: (event) => _onEnter(node),
                onExit: (event) => _onLeave(node),
                child: GestureDetector(
                  onTap: () => _onSelected(node, true),
                  onSecondaryTapDown: (details) =>
                      _onShowMenu(node, details.globalPosition),
                  child: Container(
                    width: width,
                    decoration: _boxDecoration(node),
                    child: Row(
                      children: [
                        level > 0
                            ? Padding(
                                padding: const EdgeInsets.only(left: 10.0),
                                child: Container(
                                  width: level * 15.0,
                                  height: 25.0,
                                  decoration: BoxDecoration(
                                    border: Border(
                                        left: BorderSide(
                                      color: Colors.grey.withOpacity(0.9),
                                    )),
                                  ),
                                ),
                              )
                            : Container(),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 2.0),
                          child: Row(
                            children: [
                              node.isEmpty
                                  ? Container()
                                  : const RotatedBox(
                                      quarterTurns: 1,
                                      child: Icon(
                                        Icons.chevron_right_outlined,
                                        size: 18.0,
                                      ),
                                    ),
                              const SizedBox(
                                width: 4.0,
                              ),
                              const Icon(
                                Icons.folder_outlined,
                                size: 16.0,
                              ),
                              const SizedBox(
                                width: 4.0,
                              ),
                              Text(
                                node.text,
                                overflow: TextOverflow.ellipsis,
                                style: TextStyle(
                                  fontSize: 14.0,
                                  color: Colors.white.withOpacity(0.8),
                                ),
                              )
                            ],
                          ),
                        )
                      ],
                    ),
                  ),
                ),
              ),
              ...node.children!
                  .map(
                    (e) => _buildTreeNode(e, level + 1),
                  )
                  .toList()
            ],
          );
  }
}

class TreeNode extends Equatable {
  List<TreeNode>? children;
  final String text;
  Object? data;

  bool? expand = false;
  bool? selected = false;
  bool? hover = false;
  bool? star = false;

  TreeNode(
      {required this.text,
      this.children,
      this.data,
      this.expand,
      this.selected,
      this.hover}) {
    expand ??= false;
    selected ??= false;
    hover ??= false;
    star ??= false;
  }

  void addNode(TreeNode node) {
    if (children == null) {
      children = [node];
    } else {
      children!.add(node);
    }
  }

  bool get isLeaf => children == null;

  bool get isExpand {
    if (expand == null) {
      return false;
    }
    return expand!;
  }

  bool get isEmpty => children != null && children!.isEmpty;

  @override
  List<Object?> get props => [text, data, children];
}
