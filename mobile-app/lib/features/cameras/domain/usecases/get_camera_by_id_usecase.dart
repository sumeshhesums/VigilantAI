import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/camera.dart';
import '../repositories/camera_repository.dart';

class GetCameraByIdUseCase {
  final CameraRepository _repository;

  GetCameraByIdUseCase(this._repository);

  Future<Either<Failure, Camera>> execute(String id) {
    return _repository.getCameraById(id);
  }
}
