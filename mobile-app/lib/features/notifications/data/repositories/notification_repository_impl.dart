import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../../domain/entities/notification.dart';
import '../../domain/repositories/notification_repository.dart';
import '../datasources/notification_remote_datasource.dart';
import '../models/notification_model.dart';

class NotificationRepositoryImpl implements NotificationRepository {
  final NotificationRemoteDataSource _remoteDataSource;

  NotificationRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, List<Notification>>> getNotifications({int page = 1, int pageSize = 20}) async {
    try {
      final response = await _remoteDataSource.getNotifications(page: page, pageSize: pageSize);
      final list = (response['notifications'] as List)
          .map((e) => _mapToEntity(NotificationModel.fromJson(e as Map<String, dynamic>)))
          .toList();
      return Right(list);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch notifications'));
    }
  }

  @override
  Future<Either<Failure, void>> markRead(String id) async {
    try {
      await _remoteDataSource.markRead(id);
      return const Right(null);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to mark notification as read'));
    }
  }

  @override
  Future<Either<Failure, void>> markAllRead() async {
    try {
      await _remoteDataSource.markAllRead();
      return const Right(null);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to mark all notifications as read'));
    }
  }

  Notification _mapToEntity(NotificationModel model) {
    return Notification(
      id: model.id,
      title: model.title,
      message: model.message,
      type: model.type,
      read: model.read,
      incidentId: model.incidentId,
      createdAt: DateTime.parse(model.createdAt),
    );
  }
}
