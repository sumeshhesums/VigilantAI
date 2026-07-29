import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/live_stats.dart';
import '../repositories/dashboard_repository.dart';

class GetLiveStatsUseCase {
  final DashboardRepository _repository;

  GetLiveStatsUseCase(this._repository);

  Future<Either<Failure, LiveStats>> execute() => _repository.getLiveStats();
}
