import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incident.dart';
import '../repositories/incident_repository.dart';

class CreateIncidentUseCase {
  final IncidentRepository _repository;

  CreateIncidentUseCase(this._repository);

  Future<Either<Failure, Incident>> execute({
    required String cameraId,
    required String title,
    String? description,
    required String severity,
    String? status,
  }) =>
      _repository.createIncident(
        cameraId: cameraId,
        title: title,
        description: description,
        severity: severity,
        status: status,
      );
}
