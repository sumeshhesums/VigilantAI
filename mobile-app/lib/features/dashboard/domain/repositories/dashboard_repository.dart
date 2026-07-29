import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/alert_trends.dart';
import '../entities/incidents_summary.dart';
import '../entities/kpi.dart';
import '../entities/live_stats.dart';

abstract class DashboardRepository {
  Future<Either<Failure, Kpi>> getKpis();
  Future<Either<Failure, LiveStats>> getLiveStats();
  Future<Either<Failure, AlertTrends>> getAlertTrends();
  Future<Either<Failure, IncidentsSummary>> getIncidentsSummary();
}
