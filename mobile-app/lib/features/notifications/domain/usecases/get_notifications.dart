import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/notification.dart';
import '../repositories/notification_repository.dart';

class GetNotificationsUseCase {
  final NotificationRepository _repository;

  GetNotificationsUseCase(this._repository);

  Future<Either<Failure, List<Notification>>> execute({int page = 1, int perPage = 20}) {
    return _repository.getNotifications(page: page, perPage: perPage);
  }
}
