import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/alert_trends.dart';
import '../repositories/dashboard_repository.dart';

class GetAlertTrendsUseCase {
  final DashboardRepository _repository;

  GetAlertTrendsUseCase(this._repository);

  Future<Either<Failure, AlertTrends>> execute() => _repository.getAlertTrends();
}
