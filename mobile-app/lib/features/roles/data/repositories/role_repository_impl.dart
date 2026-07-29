import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../../domain/entities/role.dart';
import '../../domain/repositories/role_repository.dart';
import '../datasources/role_remote_datasource.dart';
import '../models/role_model.dart';

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

  Role _mapToEntity(RoleModel model) {
    return Role(
      id: model.id,
      name: model.name,
      description: model.description,
    );
  }
}
