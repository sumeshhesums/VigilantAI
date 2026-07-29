import 'package:flutter/foundation.dart' show ChangeNotifier;
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../domain/entities/notification.dart' as entity;

final notificationListProvider = ChangeNotifierProvider<NotificationListNotifier>((ref) {
  return NotificationListNotifier();
});

class NotificationListNotifier extends ChangeNotifier {
  List<entity.Notification> _notifications = [];
  bool _isLoading = false;
  String? _errorMessage;
  int _unreadCount = 0;

  NotificationListNotifier();

  List<entity.Notification> get notifications => _notifications;
  bool get isLoading => _isLoading;
  String? get errorMessage => _errorMessage;
  int get unreadCount => _unreadCount;

  Future<void> loadNotifications({bool refresh = false}) async {
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    Future.delayed(const Duration(milliseconds: 500), () {
      _isLoading = false;
      _errorMessage = 'Repository not connected';
      notifyListeners();
    });
  }

  Future<void> markRead(String id) async {
    _notifications = _notifications.map((n) {
      if (n.id == id) {
        return entity.Notification(
          id: n.id,
          incidentId: n.incidentId,
          channel: n.channel,
          recipient: n.recipient,
          status: 'sent',
          attempts: n.attempts,
          responseCode: n.responseCode,
          errorMessage: n.errorMessage,
          createdAt: n.createdAt,
          sentAt: n.sentAt,
        );
      }
      return n;
    }).toList();
    _unreadCount = _notifications.where((n) => n.isUnread).length;
    notifyListeners();
  }

  Future<void> markAllRead() async {
    _notifications = _notifications.map((n) {
      return entity.Notification(
        id: n.id,
        incidentId: n.incidentId,
        channel: n.channel,
        recipient: n.recipient,
        status: 'sent',
        attempts: n.attempts,
        responseCode: n.responseCode,
        errorMessage: n.errorMessage,
        createdAt: n.createdAt,
        sentAt: n.sentAt,
      );
    }).toList();
    _unreadCount = 0;
    notifyListeners();
  }

  void clearError() {
    _errorMessage = null;
    notifyListeners();
  }
}
