// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'evidence_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

EvidenceModel _$EvidenceModelFromJson(Map<String, dynamic> json) =>
    EvidenceModel(
      id: json['id'] as String,
      incidentId: json['incident_id'] as String,
      fileName: json['file_name'] as String,
      fileType: json['file_type'] as String,
      fileSize: (json['file_size'] as num).toInt(),
      fileUrl: json['file_url'] as String,
      thumbnailUrl: json['thumbnail_url'] as String?,
      uploadedBy: json['uploaded_by'] as String,
      uploadedAt: json['uploaded_at'] as String,
      createdAt: json['created_at'] as String,
    );

Map<String, dynamic> _$EvidenceModelToJson(EvidenceModel instance) =>
    <String, dynamic>{
      'id': instance.id,
      'incident_id': instance.incidentId,
      'file_name': instance.fileName,
      'file_type': instance.fileType,
      'file_size': instance.fileSize,
      'file_url': instance.fileUrl,
      'thumbnail_url': instance.thumbnailUrl,
      'uploaded_by': instance.uploadedBy,
      'uploaded_at': instance.uploadedAt,
      'created_at': instance.createdAt,
    };
