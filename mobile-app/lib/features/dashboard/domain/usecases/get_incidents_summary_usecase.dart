import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incidents_summary.dart';
import '../repositories/dashboard_repository.dart';

class GetIncidentsSummaryUseCase {
  final DashboardRepository _repository;

  GetIncidentsSummaryUseCase(this._repository);

  Future<Either<Failure, IncidentsSummary>> execute() =>
      _repository.getIncidentsSummary();
}
