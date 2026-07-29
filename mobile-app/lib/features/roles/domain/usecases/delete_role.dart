import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../repositories/role_repository.dart';

class DeleteRoleUseCase {
  final RoleRepository _repository;

  DeleteRoleUseCase(this._repository);

  Future<Either<Failure, void>> execute(String id) {
    return _repository.deleteRole(id);
  }
}
