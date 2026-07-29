// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'update_camera_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UpdateCameraRequest _$UpdateCameraRequestFromJson(Map<String, dynamic> json) =>
    UpdateCameraRequest(
      name: json['name'] as String?,
      location: json['location'] as String?,
      rtspUrl: json['rtsp_url'] as String?,
      fps: (json['fps'] as num?)?.toInt(),
      resolution: json['resolution'] as String?,
      enabled: json['enabled'] as bool?,
    );

Map<String, dynamic> _$UpdateCameraRequestToJson(
        UpdateCameraRequest instance) =>
    <String, dynamic>{
      'name': instance.name,
      'location': instance.location,
      'rtsp_url': instance.rtspUrl,
      'fps': instance.fps,
      'resolution': instance.resolution,
      'enabled': instance.enabled,
    };
