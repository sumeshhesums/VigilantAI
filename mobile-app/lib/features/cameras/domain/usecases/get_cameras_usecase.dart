import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/camera.dart';
import '../repositories/camera_repository.dart';

class GetCamerasUseCase {
  final CameraRepository _repository;

  GetCamerasUseCase(this._repository);

  Future<Either<Failure, List<Camera>>> execute({
    int page = 1,
    int pageSize = 20,
  }) {
    return _repository.getCameras(page: page, pageSize: pageSize);
  }
}
