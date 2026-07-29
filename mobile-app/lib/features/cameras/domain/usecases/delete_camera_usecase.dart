import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../repositories/camera_repository.dart';

class DeleteCameraUseCase {
  final CameraRepository _repository;

  DeleteCameraUseCase(this._repository);

  Future<Either<Failure, void>> execute(String id) {
    return _repository.deleteCamera(id);
  }
}
