import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../repositories/notification_repository.dart';

class MarkAllReadUseCase {
  final NotificationRepository _repository;

  MarkAllReadUseCase(this._repository);

  Future<Either<Failure, void>> execute() {
    return _repository.markAllRead();
  }
}
