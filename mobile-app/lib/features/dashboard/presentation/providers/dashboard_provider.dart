import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../di/providers.dart';
import '../../domain/entities/alert_trends.dart';
import '../../domain/entities/incidents_summary.dart';
import '../../domain/entities/kpi.dart';
import '../../domain/entities/live_stats.dart';
import '../../domain/repositories/dashboard_repository.dart';

final dashboardProvider =
    ChangeNotifierProvider<DashboardNotifier>((ref) {
  final repository = ref.watch(dashboardRepositoryProvider);
  return DashboardNotifier(repository: repository);
});

class DashboardNotifier extends ChangeNotifier {
  final DashboardRepository _repository;
  bool _isLoading = false;
  Kpi? _kpis;
  LiveStats? _liveStats;
  IncidentsSummary? _incidentsSummary;
  AlertTrends? _alertTrends;
  String? _errorMessage;

  DashboardNotifier({required DashboardRepository repository})
      : _repository = repository;

  bool get isLoading => _isLoading;
  Kpi? get kpis => _kpis;
  LiveStats? get liveStats => _liveStats;
  IncidentsSummary? get incidentsSummary => _incidentsSummary;
  AlertTrends? get alertTrends => _alertTrends;
  String? get errorMessage => _errorMessage;

  Future<void> loadDashboard() async {
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    final kpisResult = await _repository.getKpis();
    kpisResult.fold(
      (failure) => _errorMessage = failure.message,
      (kpis) => _kpis = kpis,
    );

    final liveStatsResult = await _repository.getLiveStats();
    liveStatsResult.fold(
      (failure) => _errorMessage = failure.message,
      (stats) => _liveStats = stats,
    );

    final incidentsResult = await _repository.getIncidentsSummary();
    incidentsResult.fold(
      (failure) => _errorMessage = failure.message,
      (summary) => _incidentsSummary = summary,
    );

    final trendsResult = await _repository.getAlertTrends();
    trendsResult.fold(
      (failure) => _errorMessage = failure.message,
      (trends) => _alertTrends = trends,
    );

    _isLoading = false;
    notifyListeners();
  }
}
