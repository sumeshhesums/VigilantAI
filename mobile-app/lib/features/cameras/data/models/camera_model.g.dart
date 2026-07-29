// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'camera_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CameraModel _$CameraModelFromJson(Map<String, dynamic> json) => CameraModel(
      id: json['id'] as String,
      name: json['name'] as String,
      location: json['location'] as String?,
      status: json['status'] as String,
      enabled: json['enabled'] as bool,
      rtspUrl: json['rtsp_url'] as String,
      fps: (json['fps'] as num?)?.toInt(),
      resolution: json['resolution'] as String?,
      lastSeen: json['last_seen'] as String?,
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
      'rtsp_url': instance.rtspUrl,
      'fps': instance.fps,
      'resolution': instance.resolution,
      'last_seen': instance.lastSeen,
      'created_at': instance.createdAt,
      'updated_at': instance.updatedAt,
    };
