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

  IconData _channelIcon(String channel) {
    switch (channel.toLowerCase()) {
      case 'email':
        return Icons.email_outlined;
      case 'sms':
        return Icons.sms_outlined;
      case 'push':
        return Icons.notifications_outlined;
      case 'webhook':
        return Icons.webhook;
      default:
        return Icons.notifications_outlined;
    }
  }

  Color _channelColor(String channel) {
    switch (channel.toLowerCase()) {
      case 'email':
        return Colors.blue;
      case 'sms':
        return Colors.green;
      case 'push':
        return Colors.orange;
      case 'webhook':
        return Colors.purple;
      default:
        return Colors.grey;
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: CircleAvatar(
        backgroundColor: _channelColor(notification.channel).withOpacity(0.1),
        child: Icon(
          _channelIcon(notification.channel),
          color: _channelColor(notification.channel),
        ),
      ),
      title: Text(
        notification.channel,
        style: TextStyle(
          fontWeight: notification.isUnread ? FontWeight.bold : FontWeight.normal,
        ),
      ),
      subtitle: Text(
        notification.recipient,
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
