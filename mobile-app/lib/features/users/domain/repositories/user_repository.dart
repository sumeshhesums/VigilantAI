import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/user.dart';

abstract class UserRepository {
  Future<Either<Failure, List<User>>> getUsers({int page = 1, int perPage = 20});
  Future<Either<Failure, User>> getUserById(String id);
  Future<Either<Failure, User>> createUser({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
    required List<String> roles,
  });
  Future<Either<Failure, User>> updateUser(String id, {String? email, String? firstName, String? lastName});
  Future<Either<Failure, void>> deleteUser(String id);
}
