import 'package:json_annotation/json_annotation.dart';

part 'incident_model.g.dart';

@JsonSerializable()
class IncidentModel {
  final String id;
  @JsonKey(name: 'camera_id')
  final String cameraId;
  final String timestamp;
  final String severity;
  final String status;
  @JsonKey(name: 'event_type')
  final String eventType;
  final double confidence;
  @JsonKey(name: 'bounding_box')
  final Map<String, dynamic>? boundingBox;
  final Map<String, dynamic>? metadata;
  @JsonKey(name: 'created_at')
  final String createdAt;
  @JsonKey(name: 'updated_at')
  final String? updatedAt;

  const IncidentModel({
    required this.id,
    required this.cameraId,
    required this.timestamp,
    required this.severity,
    required this.status,
    required this.eventType,
    required this.confidence,
    this.boundingBox,
    this.metadata,
    required this.createdAt,
    this.updatedAt,
  });

  factory IncidentModel.fromJson(Map<String, dynamic> json) =>
      _$IncidentModelFromJson(json);

  Map<String, dynamic> toJson() => _$IncidentModelToJson(this);
}

@JsonSerializable()
class PaginatedIncidentsModel {
  final List<IncidentModel> incidents;
  final int total;
  final int page;
  @JsonKey(name: 'per_page')
  final int perPage;

  const PaginatedIncidentsModel({
    required this.incidents,
    required this.total,
    required this.page,
    required this.perPage,
  });

  factory PaginatedIncidentsModel.fromJson(Map<String, dynamic> json) =>
      _$PaginatedIncidentsModelFromJson(json);
}
