import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/role.dart';
import '../repositories/role_repository.dart';

class CreateRoleUseCase {
  final RoleRepository _repository;

  CreateRoleUseCase(this._repository);

  Future<Either<Failure, Role>> execute({
    required String name,
    String? description,
    required List<String> permissions,
  }) {
    return _repository.createRole(
      name: name,
      description: description,
      permissions: permissions,
    );
  }
}
