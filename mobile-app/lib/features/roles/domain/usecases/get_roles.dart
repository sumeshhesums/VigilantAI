import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/role.dart';
import '../repositories/role_repository.dart';

class GetRolesUseCase {
  final RoleRepository _repository;

  GetRolesUseCase(this._repository);

  Future<Either<Failure, List<Role>>> execute() {
    return _repository.getRoles();
  }
}
