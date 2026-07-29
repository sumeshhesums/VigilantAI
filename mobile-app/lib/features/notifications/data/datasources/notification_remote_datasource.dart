import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';

abstract class NotificationRemoteDataSource {
  Future<Map<String, dynamic>> getNotifications({int page = 1, int pageSize = 20});
  Future<void> markRead(String id);
  Future<void> markAllRead();
}

class NotificationRemoteDataSourceImpl implements NotificationRemoteDataSource {
  final ApiClient _client;

  NotificationRemoteDataSourceImpl(this._client);

  @override
  Future<Map<String, dynamic>> getNotifications({int page = 1, int pageSize = 20}) async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.notifications,
      queryParameters: {'page': page, 'page_size': pageSize},
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data!,
    );
  }

  @override
  Future<void> markRead(String id) async {
    final result = await _client.put<void>(
      '${ApiConstants.notificationsMarkRead}$id/read',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data,
    );
  }

  @override
  Future<void> markAllRead() async {
    final result = await _client.put<void>(
      ApiConstants.notificationsMarkAllRead,
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data,
    );
  }
}
