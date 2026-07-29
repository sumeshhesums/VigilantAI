import 'package:json_annotation/json_annotation.dart';

part 'kpi_model.g.dart';

@JsonSerializable()
class KpiModel {
  @JsonKey(name: 'active_cameras')
  final int activeCameras;
  @JsonKey(name: 'online_cameras')
  final int onlineCameras;
  @JsonKey(name: 'offline_cameras')
  final int offlineCameras;
  @JsonKey(name: 'total_detections_24h')
  final int totalDetections24h;
  @JsonKey(name: 'critical_alerts')
  final int criticalAlerts;
  @JsonKey(name: 'open_incidents')
  final int openIncidents;
  @JsonKey(name: 'avg_response_time_seconds')
  final double avgResponseTimeSeconds;
  @JsonKey(name: 'sla_compliance_percent')
  final double slaCompliancePercent;
  @JsonKey(name: 'detection_trend')
  final String detectionTrend;

  const KpiModel({
    required this.activeCameras,
    required this.onlineCameras,
    required this.offlineCameras,
    required this.totalDetections24h,
    required this.criticalAlerts,
    required this.openIncidents,
    required this.avgResponseTimeSeconds,
    required this.slaCompliancePercent,
    required this.detectionTrend,
  });

  factory KpiModel.fromJson(Map<String, dynamic> json) =>
      _$KpiModelFromJson(json);
}
