import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incident.dart';
import '../repositories/incident_repository.dart';

class GetIncidentByIdUseCase {
  final IncidentRepository _repository;

  GetIncidentByIdUseCase(this._repository);

  Future<Either<Failure, Incident>> execute(String id) =>
      _repository.getIncidentById(id);
}
