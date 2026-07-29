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
      contentType: json['content_type'] as String,
      fileSize: (json['file_size'] as num).toInt(),
      sha256: json['sha256'] as String,
      width: (json['width'] as num?)?.toInt(),
      height: (json['height'] as num?)?.toInt(),
      createdAt: json['created_at'] as String,
    );

Map<String, dynamic> _$EvidenceModelToJson(EvidenceModel instance) =>
    <String, dynamic>{
      'id': instance.id,
      'incident_id': instance.incidentId,
      'file_name': instance.fileName,
      'content_type': instance.contentType,
      'file_size': instance.fileSize,
      'sha256': instance.sha256,
      'width': instance.width,
      'height': instance.height,
      'created_at': instance.createdAt,
    };
