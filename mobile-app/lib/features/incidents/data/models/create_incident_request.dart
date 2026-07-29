import 'package:json_annotation/json_annotation.dart';

part 'create_incident_request.g.dart';

@JsonSerializable()
class CreateIncidentRequest {
  @JsonKey(name: 'camera_id')
  final String cameraId;
  final String? timestamp;
  final String severity;
  @JsonKey(name: 'event_type')
  final String eventType;
  final double confidence;
  @JsonKey(name: 'bounding_box')
  final Map<String, dynamic>? boundingBox;
  final Map<String, dynamic>? metadata;

  const CreateIncidentRequest({
    required this.cameraId,
    this.timestamp,
    required this.severity,
    required this.eventType,
    required this.confidence,
    this.boundingBox,
    this.metadata,
  });

  Map<String, dynamic> toJson() => _$CreateIncidentRequestToJson(this);
}
