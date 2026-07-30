import 'package:flutter/foundation.dart' show ChangeNotifier;
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../di/providers.dart';
import '../../domain/entities/notification.dart' as entity;
import '../../domain/repositories/notification_repository.dart' as repo;

final notificationListProvider = ChangeNotifierProvider<NotificationListNotifier>((ref) {
  return NotificationListNotifier(repository: ref.watch(notificationRepositoryProvider));
});

class NotificationListNotifier extends ChangeNotifier {
  final repo.NotificationRepository _repository;
  List<entity.Notification> _notifications = [];
  bool _isLoading = false;
  String? _errorMessage;
  int _unreadCount = 0;

  NotificationListNotifier({required repo.NotificationRepository repository})
      : _repository = repository;

  List<entity.Notification> get notifications => _notifications;
  bool get isLoading => _isLoading;
  String? get errorMessage => _errorMessage;
  int get unreadCount => _unreadCount;

  Future<void> loadNotifications({bool refresh = false}) async {
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    final result = await _repository.getNotifications(page: 1, perPage: 20);
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (list) {
        _notifications = list;
        _unreadCount = list.where((n) => n.isUnread).length;
      },
    );

    _isLoading = false;
    notifyListeners();
  }

  Future<void> markRead(String id) async {
    final result = await _repository.markRead(id);
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (_) {
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
      },
    );
    notifyListeners();
  }

  Future<void> markAllRead() async {
    final result = await _repository.markAllRead();
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (_) {
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
      },
    );
    notifyListeners();
  }

  void clearError() {
    _errorMessage = null;
    notifyListeners();
  }
}
