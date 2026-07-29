// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'create_camera_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CreateCameraRequest _$CreateCameraRequestFromJson(Map<String, dynamic> json) =>
    CreateCameraRequest(
      name: json['name'] as String,
      location: json['location'] as String,
      streamUrl: json['stream_url'] as String?,
      rtspUrl: json['rtsp_url'] as String?,
      model: json['model'] as String?,
      ipAddress: json['ip_address'] as String?,
      port: (json['port'] as num?)?.toInt(),
      username: json['username'] as String?,
      password: json['password'] as String?,
      fps: (json['fps'] as num?)?.toInt(),
      resolutionWidth: (json['resolution_width'] as num?)?.toInt(),
      resolutionHeight: (json['resolution_height'] as num?)?.toInt(),
    );

Map<String, dynamic> _$CreateCameraRequestToJson(
        CreateCameraRequest instance) =>
    <String, dynamic>{
      'name': instance.name,
      'location': instance.location,
      'stream_url': instance.streamUrl,
      'rtsp_url': instance.rtspUrl,
      'model': instance.model,
      'ip_address': instance.ipAddress,
      'port': instance.port,
      'username': instance.username,
      'password': instance.password,
      'fps': instance.fps,
      'resolution_width': instance.resolutionWidth,
      'resolution_height': instance.resolutionHeight,
    };
