import 'package:json_annotation/json_annotation.dart';

part 'update_incident_request.g.dart';

@JsonSerializable()
class UpdateIncidentRequest {
  final String? title;
  final String? description;
  final String? severity;
  final String? status;

  const UpdateIncidentRequest({
    this.title,
    this.description,
    this.severity,
    this.status,
  });

  Map<String, dynamic> toJson() => _$UpdateIncidentRequestToJson(this);
}
