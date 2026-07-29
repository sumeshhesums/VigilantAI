// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'incident_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

IncidentModel _$IncidentModelFromJson(Map<String, dynamic> json) =>
    IncidentModel(
      id: json['id'] as String,
      cameraId: json['camera_id'] as String,
      timestamp: json['timestamp'] as String,
      severity: json['severity'] as String,
      status: json['status'] as String,
      eventType: json['event_type'] as String,
      confidence: (json['confidence'] as num).toDouble(),
      boundingBox: json['bounding_box'] as Map<String, dynamic>?,
      metadata: json['metadata'] as Map<String, dynamic>?,
      createdAt: json['created_at'] as String,
      updatedAt: json['updated_at'] as String?,
    );

Map<String, dynamic> _$IncidentModelToJson(IncidentModel instance) =>
    <String, dynamic>{
      'id': instance.id,
      'camera_id': instance.cameraId,
      'timestamp': instance.timestamp,
      'severity': instance.severity,
      'status': instance.status,
      'event_type': instance.eventType,
      'confidence': instance.confidence,
      'bounding_box': instance.boundingBox,
      'metadata': instance.metadata,
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
      perPage: (json['per_page'] as num).toInt(),
    );

Map<String, dynamic> _$PaginatedIncidentsModelToJson(
        PaginatedIncidentsModel instance) =>
    <String, dynamic>{
      'incidents': instance.incidents,
      'total': instance.total,
      'page': instance.page,
      'per_page': instance.perPage,
    };
