// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'live_stats_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

LiveStatsModel _$LiveStatsModelFromJson(Map<String, dynamic> json) =>
    LiveStatsModel(
      activeAlerts: (json['active_alerts'] as num).toInt(),
      camerasOnline: (json['cameras_online'] as num).toInt(),
      camerasOffline: (json['cameras_offline'] as num).toInt(),
      detectionsToday: (json['detections_today'] as num).toInt(),
      uptimePercentage: (json['uptime_percentage'] as num).toDouble(),
      avgFps: (json['avg_fps'] as num).toDouble(),
    );

Map<String, dynamic> _$LiveStatsModelToJson(LiveStatsModel instance) =>
    <String, dynamic>{
      'active_alerts': instance.activeAlerts,
      'cameras_online': instance.camerasOnline,
      'cameras_offline': instance.camerasOffline,
      'detections_today': instance.detectionsToday,
      'uptime_percentage': instance.uptimePercentage,
      'avg_fps': instance.avgFps,
    };
