import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/role.dart';

abstract class RoleRepository {
  Future<Either<Failure, List<Role>>> getRoles();
}
