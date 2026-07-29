import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../repositories/incident_repository.dart';

class DeleteIncidentUseCase {
  final IncidentRepository _repository;

  DeleteIncidentUseCase(this._repository);

  Future<Either<Failure, void>> execute(String id) =>
      _repository.deleteIncident(id);
}
