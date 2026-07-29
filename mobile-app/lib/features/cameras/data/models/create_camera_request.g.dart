// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'create_camera_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CreateCameraRequest _$CreateCameraRequestFromJson(Map<String, dynamic> json) =>
    CreateCameraRequest(
      name: json['name'] as String,
      location: json['location'] as String?,
      rtspUrl: json['rtsp_url'] as String,
      fps: (json['fps'] as num?)?.toInt(),
      resolution: json['resolution'] as String?,
    );

Map<String, dynamic> _$CreateCameraRequestToJson(
        CreateCameraRequest instance) =>
    <String, dynamic>{
      'name': instance.name,
      'location': instance.location,
      'rtsp_url': instance.rtspUrl,
      'fps': instance.fps,
      'resolution': instance.resolution,
    };
