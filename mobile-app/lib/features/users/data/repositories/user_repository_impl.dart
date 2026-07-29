import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../../domain/entities/user.dart';
import '../../domain/repositories/user_repository.dart';
import '../datasources/user_remote_datasource.dart';
import '../models/create_user_request.dart';
import '../models/update_user_request.dart';
import '../models/user_model.dart';

class UserRepositoryImpl implements UserRepository {
  final UserRemoteDataSource _remoteDataSource;

  UserRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, List<User>>> getUsers({int page = 1, int pageSize = 20}) async {
    try {
      final response = await _remoteDataSource.getUsers(page: page, pageSize: pageSize);
      final list = (response['users'] as List)
          .map((e) => _mapToEntity(UserModel.fromJson(e as Map<String, dynamic>)))
          .toList();
      return Right(list);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch users'));
    }
  }

  @override
  Future<Either<Failure, User>> getUserById(String id) async {
    try {
      final model = await _remoteDataSource.getUserById(id);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch user'));
    }
  }

  @override
  Future<Either<Failure, User>> createUser({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
    required String role,
  }) async {
    try {
      final request = CreateUserRequest(
        email: email,
        password: password,
        firstName: firstName,
        lastName: lastName,
        role: role,
      );
      final model = await _remoteDataSource.createUser(request);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to create user'));
    }
  }

  @override
  Future<Either<Failure, User>> updateUser(
    String id, {
    String? email,
    String? firstName,
    String? lastName,
    String? role,
    bool? enabled,
  }) async {
    try {
      final request = UpdateUserRequest(
        email: email,
        firstName: firstName,
        lastName: lastName,
        role: role,
        enabled: enabled,
      );
      final model = await _remoteDataSource.updateUser(id, request);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to update user'));
    }
  }

  @override
  Future<Either<Failure, void>> deleteUser(String id) async {
    try {
      await _remoteDataSource.deleteUser(id);
      return const Right(null);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to delete user'));
    }
  }

  User _mapToEntity(UserModel model) {
    return User(
      id: model.id,
      email: model.email,
      firstName: model.firstName,
      lastName: model.lastName,
      role: model.role,
      enabled: model.enabled,
      createdAt: DateTime.parse(model.createdAt),
      updatedAt: model.updatedAt != null ? DateTime.parse(model.updatedAt!) : null,
    );
  }
}
