import 'package:flutter/material.dart';

import '../../domain/entities/role.dart';

class RoleCard extends StatelessWidget {
  final Role role;
  final VoidCallback? onTap;

  const RoleCard({
    super.key,
    required this.role,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
      child: ListTile(
        leading: CircleAvatar(
          child: Text(
            role.name[0].toUpperCase(),
            style: const TextStyle(fontWeight: FontWeight.bold),
          ),
        ),
        title: Text(role.name, style: const TextStyle(fontWeight: FontWeight.w600)),
        subtitle: role.description != null && role.description!.isNotEmpty
            ? Text(role.description!, maxLines: 1, overflow: TextOverflow.ellipsis)
            : null,
        onTap: onTap,
      ),
    );
  }
}
