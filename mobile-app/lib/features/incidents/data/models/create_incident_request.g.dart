// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'create_incident_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CreateIncidentRequest _$CreateIncidentRequestFromJson(
        Map<String, dynamic> json) =>
    CreateIncidentRequest(
      cameraId: json['camera_id'] as String,
      title: json['title'] as String,
      description: json['description'] as String?,
      severity: json['severity'] as String,
      status: json['status'] as String?,
    );

Map<String, dynamic> _$CreateIncidentRequestToJson(
        CreateIncidentRequest instance) =>
    <String, dynamic>{
      'camera_id': instance.cameraId,
      'title': instance.title,
      'description': instance.description,
      'severity': instance.severity,
      'status': instance.status,
    };
