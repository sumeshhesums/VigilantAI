import 'package:flutter/material.dart' hide Notification;

import '../../domain/entities/notification.dart' as entity;

class NotificationTile extends StatelessWidget {
  final entity.Notification notification;
  final VoidCallback? onTap;
  final VoidCallback? onMarkRead;

  const NotificationTile({
    super.key,
    required this.notification,
    this.onTap,
    this.onMarkRead,
  });

  IconData _typeIcon(String type) {
    switch (type.toLowerCase()) {
      case 'alert':
      case 'warning':
        return Icons.warning_amber;
      case 'info':
        return Icons.info_outline;
      case 'error':
        return Icons.error_outline;
      case 'success':
        return Icons.check_circle_outline;
      default:
        return Icons.notifications_outlined;
    }
  }

  Color _typeColor(String type) {
    switch (type.toLowerCase()) {
      case 'alert':
      case 'warning':
        return Colors.orange;
      case 'error':
        return Colors.red;
      case 'success':
        return Colors.green;
      case 'info':
      default:
        return Colors.blue;
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: CircleAvatar(
        backgroundColor: _typeColor(notification.type).withOpacity(0.1),
        child: Icon(
          _typeIcon(notification.type),
          color: _typeColor(notification.type),
        ),
      ),
      title: Text(
        notification.title,
        style: TextStyle(
          fontWeight: notification.isUnread ? FontWeight.bold : FontWeight.normal,
        ),
      ),
      subtitle: Text(
        notification.message,
        maxLines: 2,
        overflow: TextOverflow.ellipsis,
      ),
      trailing: notification.isUnread
          ? IconButton(
              icon: const Icon(Icons.mark_email_read_outlined, size: 20),
              onPressed: onMarkRead,
            )
          : null,
      onTap: onTap,
    );
  }
}
