import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../../domain/entities/role.dart';
import '../../domain/repositories/role_repository.dart';
import '../datasources/role_remote_datasource.dart';
import '../models/create_role_request.dart';
import '../models/role_model.dart';
import '../models/update_role_request.dart';

class RoleRepositoryImpl implements RoleRepository {
  final RoleRemoteDataSource _remoteDataSource;

  RoleRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, List<Role>>> getRoles() async {
    try {
      final models = await _remoteDataSource.getRoles();
      return Right(models.map(_mapToEntity).toList());
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch roles'));
    }
  }

  @override
  Future<Either<Failure, Role>> getRoleById(String id) async {
    try {
      final model = await _remoteDataSource.getRoleById(id);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch role'));
    }
  }

  @override
  Future<Either<Failure, Role>> createRole({
    required String name,
    String? description,
    required List<String> permissions,
  }) async {
    try {
      final request = CreateRoleRequest(
        name: name,
        description: description,
        permissions: permissions,
      );
      final model = await _remoteDataSource.createRole(request);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to create role'));
    }
  }

  @override
  Future<Either<Failure, Role>> updateRole(
    String id, {
    String? name,
    String? description,
    List<String>? permissions,
  }) async {
    try {
      final request = UpdateRoleRequest(
        name: name,
        description: description,
        permissions: permissions,
      );
      final model = await _remoteDataSource.updateRole(id, request);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to update role'));
    }
  }

  @override
  Future<Either<Failure, void>> deleteRole(String id) async {
    try {
      await _remoteDataSource.deleteRole(id);
      return const Right(null);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to delete role'));
    }
  }

  Role _mapToEntity(RoleModel model) {
    return Role(
      id: model.id,
      name: model.name,
      description: model.description,
      permissions: model.permissions,
      createdAt: DateTime.parse(model.createdAt),
      updatedAt: model.updatedAt != null ? DateTime.parse(model.updatedAt!) : null,
    );
  }
}
