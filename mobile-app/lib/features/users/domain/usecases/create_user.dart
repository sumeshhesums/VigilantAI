import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/user.dart';
import '../repositories/user_repository.dart';

class CreateUserUseCase {
  final UserRepository _repository;

  CreateUserUseCase(this._repository);

  Future<Either<Failure, User>> execute({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
    required List<String> roles,
  }) {
    return _repository.createUser(
      email: email,
      password: password,
      firstName: firstName,
      lastName: lastName,
      roles: roles,
    );
  }
}
