import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/camera.dart';
import '../repositories/camera_repository.dart';

class CreateCameraUseCase {
  final CameraRepository _repository;

  CreateCameraUseCase(this._repository);

  Future<Either<Failure, Camera>> execute({
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
  }) {
    return _repository.createCamera(
      name: name,
      location: location,
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
