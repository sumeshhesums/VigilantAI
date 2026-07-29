import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incident.dart';
import '../repositories/incident_repository.dart';

class GetIncidentsUseCase {
  final IncidentRepository _repository;

  GetIncidentsUseCase(this._repository);

  Future<Either<Failure, PaginatedIncidents>> execute({
    int page = 1,
    int perPage = 20,
  }) =>
      _repository.getIncidents(page: page, perPage: perPage);
}
