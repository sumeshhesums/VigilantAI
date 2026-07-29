import 'package:json_annotation/json_annotation.dart';

part 'live_stats_model.g.dart';

@JsonSerializable()
class LiveStatsModel {
  @JsonKey(name: 'active_alerts')
  final int activeAlerts;
  @JsonKey(name: 'cameras_online')
  final int camerasOnline;
  @JsonKey(name: 'cameras_offline')
  final int camerasOffline;
  @JsonKey(name: 'detections_today')
  final int detectionsToday;
  @JsonKey(name: 'uptime_percentage')
  final double uptimePercentage;
  @JsonKey(name: 'avg_fps')
  final double avgFps;

  const LiveStatsModel({
    required this.activeAlerts,
    required this.camerasOnline,
    required this.camerasOffline,
    required this.detectionsToday,
    required this.uptimePercentage,
    required this.avgFps,
  });

  factory LiveStatsModel.fromJson(Map<String, dynamic> json) =>
      _$LiveStatsModelFromJson(json);
}
