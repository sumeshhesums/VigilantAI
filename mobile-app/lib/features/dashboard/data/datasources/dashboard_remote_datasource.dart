import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/alert_trends_model.dart';
import '../models/incidents_summary_model.dart';
import '../models/kpi_model.dart';
import '../models/live_stats_model.dart';

abstract class DashboardRemoteDataSource {
  Future<KpiModel> getKpis();
  Future<LiveStatsModel> getLiveStats();
  Future<AlertTrendsModel> getAlertTrends();
  Future<IncidentsSummaryModel> getIncidentsSummary();
}

class DashboardRemoteDataSourceImpl implements DashboardRemoteDataSource {
  final ApiClient _client;

  DashboardRemoteDataSourceImpl(this._client);

  @override
  Future<KpiModel> getKpis() async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.dashboardKpis,
    );
    return result.fold(
      (failure) => throw failure,
      (response) => KpiModel.fromJson(response.data!),
    );
  }

  @override
  Future<LiveStatsModel> getLiveStats() async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.dashboardLiveStats,
    );
    return result.fold(
      (failure) => throw failure,
      (response) => LiveStatsModel.fromJson(response.data!),
    );
  }

  @override
  Future<AlertTrendsModel> getAlertTrends() async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.dashboardAlertTrends,
    );
    return result.fold(
      (failure) => throw failure,
      (response) => AlertTrendsModel.fromJson(response.data!),
    );
  }

  @override
  Future<IncidentsSummaryModel> getIncidentsSummary() async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.dashboardIncidentsSummary,
    );
    return result.fold(
      (failure) => throw failure,
      (response) => IncidentsSummaryModel.fromJson(response.data!),
    );
  }
}
