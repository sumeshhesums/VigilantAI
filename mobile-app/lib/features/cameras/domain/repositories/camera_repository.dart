import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/camera.dart';

abstract class CameraRepository {
  Future<Either<Failure, List<Camera>>> getCameras({
    int page = 1,
    int pageSize = 20,
  });
  Future<Either<Failure, Camera>> getCameraById(String id);
  Future<Either<Failure, Camera>> createCamera({
    required String name,
    required String location,
    String? streamUrl,
    String? rtspUrl,
    String? model,
    String? ipAddress,
    int? port,
    String? password,
    String? username,
    int? fps,
    int? resolutionWidth,
    int? resolutionHeight,
  });
  Future<Either<Failure, Camera>> updateCamera(
    String id, {
    String? name,
    String? location,
    bool? enabled,
    String? streamUrl,
    String? rtspUrl,
    String? model,
    String? ipAddress,
    int? port,
    String? password,
    String? username,
    int? fps,
    int? resolutionWidth,
    int? resolutionHeight,
  });
  Future<Either<Failure, void>> deleteCamera(String id);
}
