import 'package:json_annotation/json_annotation.dart';

part 'incident_model.g.dart';

@JsonSerializable()
class IncidentModel {
  final String id;
  @JsonKey(name: 'camera_id')
  final String cameraId;
  @JsonKey(name: 'camera_name')
  final String cameraName;
  final String title;
  final String description;
  final String severity;
  final String status;
  @JsonKey(name: 'detected_at')
  final String detectedAt;
  @JsonKey(name: 'acknowledged_at')
  final String? acknowledgedAt;
  @JsonKey(name: 'resolved_at')
  final String? resolvedAt;
  @JsonKey(name: 'created_at')
  final String createdAt;
  @JsonKey(name: 'updated_at')
  final String updatedAt;

  const IncidentModel({
    required this.id,
    required this.cameraId,
    required this.cameraName,
    required this.title,
    required this.description,
    required this.severity,
    required this.status,
    required this.detectedAt,
    this.acknowledgedAt,
    this.resolvedAt,
    required this.createdAt,
    required this.updatedAt,
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
  @JsonKey(name: 'page_size')
  final int pageSize;

  const PaginatedIncidentsModel({
    required this.incidents,
    required this.total,
    required this.page,
    required this.pageSize,
  });

  factory PaginatedIncidentsModel.fromJson(Map<String, dynamic> json) =>
      _$PaginatedIncidentsModelFromJson(json);
}
