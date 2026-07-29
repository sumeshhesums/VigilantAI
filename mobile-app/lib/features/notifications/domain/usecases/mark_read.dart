import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../repositories/notification_repository.dart';

class MarkReadUseCase {
  final NotificationRepository _repository;

  MarkReadUseCase(this._repository);

  Future<Either<Failure, void>> execute(String id) {
    return _repository.markRead(id);
  }
}
