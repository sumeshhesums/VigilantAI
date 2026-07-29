import 'package:json_annotation/json_annotation.dart';

part 'evidence_model.g.dart';

@JsonSerializable()
class EvidenceModel {
  final String id;
  @JsonKey(name: 'incident_id')
  final String incidentId;
  @JsonKey(name: 'file_name')
  final String fileName;
  @JsonKey(name: 'content_type')
  final String contentType;
  @JsonKey(name: 'file_size')
  final int fileSize;
  final String sha256;
  final int? width;
  final int? height;
  @JsonKey(name: 'created_at')
  final String createdAt;

  const EvidenceModel({
    required this.id,
    required this.incidentId,
    required this.fileName,
    required this.contentType,
    required this.fileSize,
    required this.sha256,
    this.width,
    this.height,
    required this.createdAt,
  });

  factory EvidenceModel.fromJson(Map<String, dynamic> json) =>
      _$EvidenceModelFromJson(json);
}
