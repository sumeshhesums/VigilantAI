import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../../domain/entities/alert_trends.dart';
import '../../domain/entities/incidents_summary.dart';
import '../../domain/entities/kpi.dart';
import '../../domain/entities/live_stats.dart';
import '../../domain/repositories/dashboard_repository.dart';
import '../datasources/dashboard_remote_datasource.dart';

class DashboardRepositoryImpl implements DashboardRepository {
  final DashboardRemoteDataSource _remoteDataSource;

  DashboardRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, Kpi>> getKpis() async {
    try {
      final model = await _remoteDataSource.getKpis();
      return Right(Kpi(
        activeCameras: model.activeCameras,
        onlineCameras: model.onlineCameras,
        offlineCameras: model.offlineCameras,
        totalDetections24h: model.totalDetections24h,
        criticalAlerts: model.criticalAlerts,
        openIncidents: model.openIncidents,
        avgResponseTimeSeconds: model.avgResponseTimeSeconds,
        slaCompliancePercent: model.slaCompliancePercent,
        detectionTrend: model.detectionTrend,
      ));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch KPIs'));
    }
  }

  @override
  Future<Either<Failure, LiveStats>> getLiveStats() async {
    try {
      final model = await _remoteDataSource.getLiveStats();
      return Right(LiveStats(
        activeAlerts: model.activeAlerts,
        camerasOnline: model.camerasOnline,
        camerasOffline: model.camerasOffline,
        detectionsToday: model.detectionsToday,
        uptimePercentage: model.uptimePercentage,
        avgFps: model.avgFps,
      ));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch live stats'));
    }
  }

  @override
  Future<Either<Failure, AlertTrends>> getAlertTrends() async {
    try {
      final model = await _remoteDataSource.getAlertTrends();
      return Right(AlertTrends(
        interval: model.interval,
        series: model.series
            .map((p) => AlertTrendPoint(
                  timestamp: p.timestamp,
                  critical: p.critical,
                  high: p.high,
                  medium: p.medium,
                  low: p.low,
                ))
            .toList(),
      ));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch alert trends'));
    }
  }

  @override
  Future<Either<Failure, IncidentsSummary>> getIncidentsSummary() async {
    try {
      final model = await _remoteDataSource.getIncidentsSummary();
      return Right(IncidentsSummary(
        total: model.total,
        byStatus: model.byStatus
            .map((s) => IncidentSummaryItem(
                  status: s.status,
                  count: s.count,
                  percentage: s.percentage,
                ))
            .toList(),
        bySeverity: model.bySeverity
            .map((s) => IncidentSummaryItem(
                  status: s.status,
                  count: s.count,
                  percentage: s.percentage,
                ))
            .toList(),
      ));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch incidents summary'));
    }
  }
}
