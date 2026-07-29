import 'package:json_annotation/json_annotation.dart';

part 'create_incident_request.g.dart';

@JsonSerializable()
class CreateIncidentRequest {
  @JsonKey(name: 'camera_id')
  final String cameraId;
  final String title;
  final String? description;
  final String severity;
  final String? status;

  const CreateIncidentRequest({
    required this.cameraId,
    required this.title,
    this.description,
    required this.severity,
    this.status,
  });

  Map<String, dynamic> toJson() => _$CreateIncidentRequestToJson(this);
}
