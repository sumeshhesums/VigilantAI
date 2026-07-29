import 'package:json_annotation/json_annotation.dart';

part 'update_incident_request.g.dart';

@JsonSerializable()
class UpdateIncidentRequest {
  final String status;

  const UpdateIncidentRequest({
    required this.status,
  });

  Map<String, dynamic> toJson() => _$UpdateIncidentRequestToJson(this);
}
