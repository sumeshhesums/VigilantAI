import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/user.dart';
import '../repositories/user_repository.dart';

class GetUsersUseCase {
  final UserRepository _repository;

  GetUsersUseCase(this._repository);

  Future<Either<Failure, List<User>>> execute({int page = 1, int perPage = 20}) {
    return _repository.getUsers(page: page, perPage: perPage);
  }
}
