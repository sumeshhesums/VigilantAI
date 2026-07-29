// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'update_incident_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UpdateIncidentRequest _$UpdateIncidentRequestFromJson(
        Map<String, dynamic> json) =>
    UpdateIncidentRequest(
      title: json['title'] as String?,
      description: json['description'] as String?,
      severity: json['severity'] as String?,
      status: json['status'] as String?,
    );

Map<String, dynamic> _$UpdateIncidentRequestToJson(
        UpdateIncidentRequest instance) =>
    <String, dynamic>{
      'title': instance.title,
      'description': instance.description,
      'severity': instance.severity,
      'status': instance.status,
    };
