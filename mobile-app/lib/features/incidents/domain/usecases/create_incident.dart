import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incident.dart';
import '../repositories/incident_repository.dart';

class CreateIncidentUseCase {
  final IncidentRepository _repository;

  CreateIncidentUseCase(this._repository);

  Future<Either<Failure, Incident>> execute({
    required String cameraId,
    required String severity,
    required String eventType,
    required double confidence,
    Map<String, dynamic>? boundingBox,
    Map<String, dynamic>? metadata,
  }) =>
      _repository.createIncident(
        cameraId: cameraId,
        severity: severity,
        eventType: eventType,
        confidence: confidence,
        boundingBox: boundingBox,
        metadata: metadata,
      );
}
