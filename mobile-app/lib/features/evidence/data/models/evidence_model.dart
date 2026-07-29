import 'package:json_annotation/json_annotation.dart';

part 'evidence_model.g.dart';

@JsonSerializable()
class EvidenceModel {
  final String id;
  @JsonKey(name: 'incident_id')
  final String incidentId;
  @JsonKey(name: 'file_name')
  final String fileName;
  @JsonKey(name: 'file_type')
  final String fileType;
  @JsonKey(name: 'file_size')
  final int fileSize;
  @JsonKey(name: 'file_url')
  final String fileUrl;
  @JsonKey(name: 'thumbnail_url')
  final String? thumbnailUrl;
  @JsonKey(name: 'uploaded_by')
  final String uploadedBy;
  @JsonKey(name: 'uploaded_at')
  final String uploadedAt;
  @JsonKey(name: 'created_at')
  final String createdAt;

  const EvidenceModel({
    required this.id,
    required this.incidentId,
    required this.fileName,
    required this.fileType,
    required this.fileSize,
    required this.fileUrl,
    this.thumbnailUrl,
    required this.uploadedBy,
    required this.uploadedAt,
    required this.createdAt,
  });

  factory EvidenceModel.fromJson(Map<String, dynamic> json) =>
      _$EvidenceModelFromJson(json);
}
