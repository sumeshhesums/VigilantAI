import 'package:json_annotation/json_annotation.dart';

part 'update_camera_request.g.dart';

@JsonSerializable()
class UpdateCameraRequest {
  final String? name;
  final String? location;
  @JsonKey(name: 'rtsp_url')
  final String? rtspUrl;
  final int? fps;
  final String? resolution;
  final bool? enabled;

  const UpdateCameraRequest({
    this.name,
    this.location,
    this.rtspUrl,
    this.fps,
    this.resolution,
    this.enabled,
  });

  Map<String, dynamic> toJson() => _$UpdateCameraRequestToJson(this);
}
