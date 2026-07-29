import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incident.dart';
import '../repositories/incident_repository.dart';

class UpdateIncidentUseCase {
  final IncidentRepository _repository;

  UpdateIncidentUseCase(this._repository);

  Future<Either<Failure, Incident>> execute(
    String id, {
    required String status,
  }) =>
      _repository.updateIncident(
        id,
        status: status,
      );
}
