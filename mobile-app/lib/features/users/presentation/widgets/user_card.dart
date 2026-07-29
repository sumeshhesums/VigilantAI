import 'package:flutter/material.dart';

import '../../domain/entities/user.dart';

class UserCard extends StatelessWidget {
  final User user;
  final VoidCallback? onTap;
  final VoidCallback? onDelete;

  const UserCard({
    super.key,
    required this.user,
    this.onTap,
    this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      child: ListTile(
        leading: CircleAvatar(
          child: Text(
            '${user.firstName[0]}${user.lastName[0]}',
            style: const TextStyle(fontWeight: FontWeight.bold),
          ),
        ),
        title: Text(user.fullName, style: const TextStyle(fontWeight: FontWeight.w600)),
        subtitle: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(user.email),
            const SizedBox(height: 2),
            Row(
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                  decoration: BoxDecoration(
                    color: Theme.of(context).colorScheme.primaryContainer,
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Text(
                    user.roles.join(', '),
                    style: TextStyle(
                      fontSize: 11,
                      color: Theme.of(context).colorScheme.onPrimaryContainer,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                Icon(
                  user.isActive ? Icons.check_circle : Icons.cancel,
                  size: 14,
                  color: user.isActive ? Colors.green : Colors.red,
                ),
                const SizedBox(width: 4),
                Text(
                  user.isActive ? 'Active' : 'Inactive',
                  style: TextStyle(fontSize: 11, color: user.isActive ? Colors.green : Colors.red),
                ),
              ],
            ),
          ],
        ),
        trailing: onDelete != null
            ? IconButton(
                icon: const Icon(Icons.delete_outline),
                onPressed: onDelete,
                color: Colors.red,
              )
            : null,
        onTap: onTap,
      ),
    );
  }
}
