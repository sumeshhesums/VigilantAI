import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/camera.dart';
import '../repositories/camera_repository.dart';

class UpdateCameraUseCase {
  final CameraRepository _repository;

  UpdateCameraUseCase(this._repository);

  Future<Either<Failure, Camera>> execute(
    String id, {
    String? name,
    String? location,
    bool? enabled,
    String? rtspUrl,
    int? fps,
    String? resolution,
  }) {
    return _repository.updateCamera(
      id,
      name: name,
      location: location,
      enabled: enabled,
      rtspUrl: rtspUrl,
      fps: fps,
      resolution: resolution,
    );
  }
}
