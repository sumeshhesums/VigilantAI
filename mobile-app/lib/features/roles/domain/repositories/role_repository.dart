import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/role.dart';

abstract class RoleRepository {
  Future<Either<Failure, List<Role>>> getRoles();
  Future<Either<Failure, Role>> getRoleById(String id);
  Future<Either<Failure, Role>> createRole({
    required String name,
    String? description,
    required List<String> permissions,
  });
  Future<Either<Failure, Role>> updateRole(String id, {String? name, String? description, List<String>? permissions});
  Future<Either<Failure, void>> deleteRole(String id);
}
