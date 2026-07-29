import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/camera.dart';

abstract class CameraRepository {
  Future<Either<Failure, List<Camera>>> getCameras({
    int page = 1,
    int perPage = 20,
  });
  Future<Either<Failure, Camera>> getCameraById(String id);
  Future<Either<Failure, Camera>> createCamera({
    required String name,
    String? location,
    required String rtspUrl,
    int? fps,
    String? resolution,
  });
  Future<Either<Failure, Camera>> updateCamera(
    String id, {
    String? name,
    String? location,
    String? rtspUrl,
    int? fps,
    String? resolution,
    bool? enabled,
  });
  Future<Either<Failure, void>> deleteCamera(String id);
}
