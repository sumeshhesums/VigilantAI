// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'create_incident_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CreateIncidentRequest _$CreateIncidentRequestFromJson(
        Map<String, dynamic> json) =>
    CreateIncidentRequest(
      cameraId: json['camera_id'] as String,
      timestamp: json['timestamp'] as String?,
      severity: json['severity'] as String,
      eventType: json['event_type'] as String,
      confidence: (json['confidence'] as num).toDouble(),
      boundingBox: json['bounding_box'] as Map<String, dynamic>?,
      metadata: json['metadata'] as Map<String, dynamic>?,
    );

Map<String, dynamic> _$CreateIncidentRequestToJson(
        CreateIncidentRequest instance) =>
    <String, dynamic>{
      'camera_id': instance.cameraId,
      'timestamp': instance.timestamp,
      'severity': instance.severity,
      'event_type': instance.eventType,
      'confidence': instance.confidence,
      'bounding_box': instance.boundingBox,
      'metadata': instance.metadata,
    };
