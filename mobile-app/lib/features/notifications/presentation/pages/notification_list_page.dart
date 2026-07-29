import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/notification_provider.dart';
import '../widgets/notification_tile.dart';

class NotificationListPage extends ConsumerWidget {
  const NotificationListPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifier = ref.watch(notificationListProvider);
    final state = ref.read(notificationListProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Notifications'),
        actions: [
          if (notifier.unreadCount > 0)
            TextButton(
              onPressed: () => state.markAllRead(),
              child: const Text('Mark all read'),
            ),
        ],
      ),
      body: notifier.isLoading
          ? const Center(child: CircularProgressIndicator())
          : notifier.errorMessage != null
              ? Center(
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(notifier.errorMessage!),
                      const SizedBox(height: 8),
                      ElevatedButton(
                        onPressed: () => state.loadNotifications(refresh: true),
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                )
              : notifier.notifications.isEmpty
                  ? const Center(child: Text('No notifications'))
                  : ListView.separated(
                      itemCount: notifier.notifications.length,
                      separatorBuilder: (_, __) => const Divider(height: 1),
                      itemBuilder: (context, index) {
                        final item = notifier.notifications[index];
                        return NotificationTile(
                          notification: item,
                          onMarkRead: () => state.markRead(item.id),
                        );
                      },
                    ),
    );
  }
}
