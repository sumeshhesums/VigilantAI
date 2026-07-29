import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/user.dart';

abstract class UserRepository {
  Future<Either<Failure, List<User>>> getUsers({int page = 1, int pageSize = 20});
  Future<Either<Failure, User>> getUserById(String id);
  Future<Either<Failure, User>> createUser({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
    required String role,
  });
  Future<Either<Failure, User>> updateUser(String id, {String? email, String? firstName, String? lastName, String? role, bool? enabled});
  Future<Either<Failure, void>> deleteUser(String id);
}
