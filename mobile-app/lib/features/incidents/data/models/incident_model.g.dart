// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'incident_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

IncidentModel _$IncidentModelFromJson(Map<String, dynamic> json) =>
    IncidentModel(
      id: json['id'] as String,
      cameraId: json['camera_id'] as String,
      cameraName: json['camera_name'] as String,
      title: json['title'] as String,
      description: json['description'] as String,
      severity: json['severity'] as String,
      status: json['status'] as String,
      detectedAt: json['detected_at'] as String,
      acknowledgedAt: json['acknowledged_at'] as String?,
      resolvedAt: json['resolved_at'] as String?,
      createdAt: json['created_at'] as String,
      updatedAt: json['updated_at'] as String,
    );

Map<String, dynamic> _$IncidentModelToJson(IncidentModel instance) =>
    <String, dynamic>{
      'id': instance.id,
      'camera_id': instance.cameraId,
      'camera_name': instance.cameraName,
      'title': instance.title,
      'description': instance.description,
      'severity': instance.severity,
      'status': instance.status,
      'detected_at': instance.detectedAt,
      'acknowledged_at': instance.acknowledgedAt,
      'resolved_at': instance.resolvedAt,
      'created_at': instance.createdAt,
      'updated_at': instance.updatedAt,
    };

PaginatedIncidentsModel _$PaginatedIncidentsModelFromJson(
        Map<String, dynamic> json) =>
    PaginatedIncidentsModel(
      incidents: (json['incidents'] as List<dynamic>)
          .map((e) => IncidentModel.fromJson(e as Map<String, dynamic>))
          .toList(),
      total: (json['total'] as num).toInt(),
      page: (json['page'] as num).toInt(),
      pageSize: (json['page_size'] as num).toInt(),
    );

Map<String, dynamic> _$PaginatedIncidentsModelToJson(
        PaginatedIncidentsModel instance) =>
    <String, dynamic>{
      'incidents': instance.incidents,
      'total': instance.total,
      'page': instance.page,
      'page_size': instance.pageSize,
    };
