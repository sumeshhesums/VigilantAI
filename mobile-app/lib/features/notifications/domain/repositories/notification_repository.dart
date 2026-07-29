import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/notification.dart';

abstract class NotificationRepository {
  Future<Either<Failure, List<Notification>>> getNotifications({int page = 1, int perPage = 20});
  Future<Either<Failure, void>> markRead(String id);
  Future<Either<Failure, void>> markAllRead();
}
