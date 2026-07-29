import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incident.dart';
import '../repositories/incident_repository.dart';

class UpdateIncidentUseCase {
  final IncidentRepository _repository;

  UpdateIncidentUseCase(this._repository);

  Future<Either<Failure, Incident>> execute(
    String id, {
    String? title,
    String? description,
    String? severity,
    String? status,
  }) =>
      _repository.updateIncident(
        id,
        title: title,
        description: description,
        severity: severity,
        status: status,
      );
}
