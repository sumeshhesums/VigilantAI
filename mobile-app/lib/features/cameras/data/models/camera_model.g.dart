// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'camera_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CameraModel _$CameraModelFromJson(Map<String, dynamic> json) => CameraModel(
      id: json['id'] as String,
      name: json['name'] as String,
      location: json['location'] as String,
      status: json['status'] as String,
      enabled: json['enabled'] as bool,
      streamUrl: json['stream_url'] as String?,
      rtspUrl: json['rtsp_url'] as String?,
      model: json['model'] as String?,
      ipAddress: json['ip_address'] as String?,
      port: (json['port'] as num?)?.toInt(),
      username: json['username'] as String?,
      fps: (json['fps'] as num?)?.toInt(),
      resolutionWidth: (json['resolution_width'] as num?)?.toInt(),
      resolutionHeight: (json['resolution_height'] as num?)?.toInt(),
      createdAt: json['created_at'] as String,
      updatedAt: json['updated_at'] as String?,
    );

Map<String, dynamic> _$CameraModelToJson(CameraModel instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'location': instance.location,
      'status': instance.status,
      'enabled': instance.enabled,
      'stream_url': instance.streamUrl,
      'rtsp_url': instance.rtspUrl,
      'model': instance.model,
      'ip_address': instance.ipAddress,
      'port': instance.port,
      'username': instance.username,
      'fps': instance.fps,
      'resolution_width': instance.resolutionWidth,
      'resolution_height': instance.resolutionHeight,
      'created_at': instance.createdAt,
      'updated_at': instance.updatedAt,
    };
