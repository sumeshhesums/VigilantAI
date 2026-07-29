import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/user.dart';
import '../repositories/user_repository.dart';

class UpdateUserUseCase {
  final UserRepository _repository;

  UpdateUserUseCase(this._repository);

  Future<Either<Failure, User>> execute(String id, {String? email, String? firstName, String? lastName, String? role, bool? enabled}) {
    return _repository.updateUser(id, email: email, firstName: firstName, lastName: lastName, role: role, enabled: enabled);
  }
}
