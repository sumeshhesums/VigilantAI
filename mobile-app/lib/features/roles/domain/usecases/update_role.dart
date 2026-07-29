import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/role.dart';
import '../repositories/role_repository.dart';

class UpdateRoleUseCase {
  final RoleRepository _repository;

  UpdateRoleUseCase(this._repository);

  Future<Either<Failure, Role>> execute(String id, {String? name, String? description, List<String>? permissions}) {
    return _repository.updateRole(id, name: name, description: description, permissions: permissions);
  }
}
