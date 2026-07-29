import 'package:json_annotation/json_annotation.dart';

import '../../domain/entities/camera.dart';

part 'camera_model.g.dart';

@JsonSerializable()
class CameraModel {
  final String id;
  final String name;
  final String? location;
  final String status;
  final bool enabled;
  @JsonKey(name: 'rtsp_url')
  final String rtspUrl;
  final int? fps;
  final String? resolution;
  @JsonKey(name: 'last_seen')
  final String? lastSeen;
  @JsonKey(name: 'created_at')
  final String createdAt;
  @JsonKey(name: 'updated_at')
  final String? updatedAt;

  const CameraModel({
    required this.id,
    required this.name,
    this.location,
    required this.status,
    required this.enabled,
    required this.rtspUrl,
    this.fps,
    this.resolution,
    this.lastSeen,
    required this.createdAt,
    this.updatedAt,
  });

  factory CameraModel.fromJson(Map<String, dynamic> json) =>
      _$CameraModelFromJson(json);

  Map<String, dynamic> toJson() => _$CameraModelToJson(this);

  Camera toEntity() => Camera(
        id: id,
        name: name,
        location: location,
        status: status,
        enabled: enabled,
        rtspUrl: rtspUrl,
        fps: fps,
        resolution: resolution,
        lastSeen: lastSeen != null ? DateTime.parse(lastSeen!) : null,
        createdAt: DateTime.parse(createdAt),
        updatedAt: updatedAt != null ? DateTime.parse(updatedAt!) : null,
      );
}
