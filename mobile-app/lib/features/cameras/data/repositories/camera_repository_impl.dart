import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../../domain/entities/camera.dart';
import '../../domain/repositories/camera_repository.dart';
import '../datasources/camera_remote_datasource.dart';
import '../models/create_camera_request.dart';
import '../models/update_camera_request.dart';

class CameraRepositoryImpl implements CameraRepository {
  final CameraRemoteDataSource _remoteDataSource;

  CameraRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, List<Camera>>> getCameras({
    int page = 1,
    int perPage = 20,
  }) async {
    try {
      final response = await _remoteDataSource.getCameras(
        page: page,
        perPage: perPage,
      );
      return Right(response.cameras.map((m) => m.toEntity()).toList());
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to get cameras'));
    }
  }

  @override
  Future<Either<Failure, Camera>> getCameraById(String id) async {
    try {
      final response = await _remoteDataSource.getCameraById(id);
      return Right(response.toEntity());
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to get camera'));
    }
  }

  @override
  Future<Either<Failure, Camera>> createCamera({
    required String name,
    String? location,
    required String rtspUrl,
    int? fps,
    String? resolution,
  }) async {
    try {
      final request = CreateCameraRequest(
        name: name,
        location: location,
        rtspUrl: rtspUrl,
        fps: fps,
        resolution: resolution,
      );
      final response = await _remoteDataSource.createCamera(request);
      return Right(response.toEntity());
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to create camera'));
    }
  }

  @override
  Future<Either<Failure, Camera>> updateCamera(
    String id, {
    String? name,
    String? location,
    String? rtspUrl,
    int? fps,
    String? resolution,
    bool? enabled,
  }) async {
    try {
      final request = UpdateCameraRequest(
        name: name,
        location: location,
        rtspUrl: rtspUrl,
        fps: fps,
        resolution: resolution,
        enabled: enabled,
      );
      final response = await _remoteDataSource.updateCamera(id, request);
      return Right(response.toEntity());
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to update camera'));
    }
  }

  @override
  Future<Either<Failure, void>> deleteCamera(String id) async {
    try {
      await _remoteDataSource.deleteCamera(id);
      return const Right(null);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to delete camera'));
    }
  }
}
