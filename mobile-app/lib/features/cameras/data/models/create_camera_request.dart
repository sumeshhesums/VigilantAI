import 'package:json_annotation/json_annotation.dart';

part 'create_camera_request.g.dart';

@JsonSerializable()
class CreateCameraRequest {
  final String name;
  final String? location;
  @JsonKey(name: 'rtsp_url')
  final String rtspUrl;
  final int? fps;
  final String? resolution;

  const CreateCameraRequest({
    required this.name,
    this.location,
    required this.rtspUrl,
    this.fps,
    this.resolution,
  });

  Map<String, dynamic> toJson() => _$CreateCameraRequestToJson(this);
}
