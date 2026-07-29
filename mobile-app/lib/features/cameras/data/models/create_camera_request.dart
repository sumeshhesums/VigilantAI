import 'package:json_annotation/json_annotation.dart';

part 'create_camera_request.g.dart';

@JsonSerializable()
class CreateCameraRequest {
  final String name;
  final String location;
  @JsonKey(name: 'stream_url')
  final String? streamUrl;
  @JsonKey(name: 'rtsp_url')
  final String? rtspUrl;
  final String? model;
  @JsonKey(name: 'ip_address')
  final String? ipAddress;
  final int? port;
  final String? username;
  final String? password;
  final int? fps;
  @JsonKey(name: 'resolution_width')
  final int? resolutionWidth;
  @JsonKey(name: 'resolution_height')
  final int? resolutionHeight;

  const CreateCameraRequest({
    required this.name,
    required this.location,
    this.streamUrl,
    this.rtspUrl,
    this.model,
    this.ipAddress,
    this.port,
    this.username,
    this.password,
    this.fps,
    this.resolutionWidth,
    this.resolutionHeight,
  });

  Map<String, dynamic> toJson() => _$CreateCameraRequestToJson(this);
}
