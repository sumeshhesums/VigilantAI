import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/camera.dart';
import '../repositories/camera_repository.dart';

class CreateCameraUseCase {
  final CameraRepository _repository;

  CreateCameraUseCase(this._repository);

  Future<Either<Failure, Camera>> execute({
    required String name,
    String? location,
    required String rtspUrl,
    int? fps,
    String? resolution,
  }) {
    return _repository.createCamera(
      name: name,
      location: location,
      rtspUrl: rtspUrl,
      fps: fps,
      resolution: resolution,
    );
  }
}
