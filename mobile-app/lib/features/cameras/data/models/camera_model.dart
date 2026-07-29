import 'package:json_annotation/json_annotation.dart';

import '../../domain/entities/camera.dart';

part 'camera_model.g.dart';

@JsonSerializable()
class CameraModel {
  final String id;
  final String name;
  final String location;
  final String status;
  final bool enabled;
  @JsonKey(name: 'stream_url')
  final String? streamUrl;
  @JsonKey(name: 'rtsp_url')
  final String? rtspUrl;
  final String? model;
  @JsonKey(name: 'ip_address')
  final String? ipAddress;
  final int? port;
  final String? username;
  final int? fps;
  @JsonKey(name: 'resolution_width')
  final int? resolutionWidth;
  @JsonKey(name: 'resolution_height')
  final int? resolutionHeight;
  @JsonKey(name: 'created_at')
  final String createdAt;
  @JsonKey(name: 'updated_at')
  final String? updatedAt;

  const CameraModel({
    required this.id,
    required this.name,
    required this.location,
    required this.status,
    required this.enabled,
    this.streamUrl,
    this.rtspUrl,
    this.model,
    this.ipAddress,
    this.port,
    this.username,
    this.fps,
    this.resolutionWidth,
    this.resolutionHeight,
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
        streamUrl: streamUrl,
        rtspUrl: rtspUrl,
        model: model,
        ipAddress: ipAddress,
        port: port,
        username: username,
        fps: fps,
        resolutionWidth: resolutionWidth,
        resolutionHeight: resolutionHeight,
        createdAt: DateTime.parse(createdAt),
        updatedAt: updatedAt != null ? DateTime.parse(updatedAt!) : null,
      );
}
