// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'kpi_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

KpiModel _$KpiModelFromJson(Map<String, dynamic> json) => KpiModel(
      activeCameras: (json['active_cameras'] as num).toInt(),
      onlineCameras: (json['online_cameras'] as num).toInt(),
      offlineCameras: (json['offline_cameras'] as num).toInt(),
      totalDetections24h: (json['total_detections_24h'] as num).toInt(),
      criticalAlerts: (json['critical_alerts'] as num).toInt(),
      openIncidents: (json['open_incidents'] as num).toInt(),
      avgResponseTimeSeconds:
          (json['avg_response_time_seconds'] as num).toDouble(),
      slaCompliancePercent: (json['sla_compliance_percent'] as num).toDouble(),
      detectionTrend: json['detection_trend'] as String,
    );

Map<String, dynamic> _$KpiModelToJson(KpiModel instance) => <String, dynamic>{
      'active_cameras': instance.activeCameras,
      'online_cameras': instance.onlineCameras,
      'offline_cameras': instance.offlineCameras,
      'total_detections_24h': instance.totalDetections24h,
      'critical_alerts': instance.criticalAlerts,
      'open_incidents': instance.openIncidents,
      'avg_response_time_seconds': instance.avgResponseTimeSeconds,
      'sla_compliance_percent': instance.slaCompliancePercent,
      'detection_trend': instance.detectionTrend,
    };
