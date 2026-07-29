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
  }) {
    return _repository.updateCamera(
      id,
      name: name,
      location: location,
      enabled: enabled,
      streamUrl: streamUrl,
      rtspUrl: rtspUrl,
      model: model,
      ipAddress: ipAddress,
      port: port,
      password: password,
      username: username,
      fps: fps,
      resolutionWidth: resolutionWidth,
      resolutionHeight: resolutionHeight,
    );
  }
}
