// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'notification_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

NotificationModel _$NotificationModelFromJson(Map<String, dynamic> json) =>
    NotificationModel(
      id: json['id'] as String,
      incidentId: json['incident_id'] as String,
      channel: json['channel'] as String,
      recipient: json['recipient'] as String,
      status: json['status'] as String,
      attempts: (json['attempts'] as num).toInt(),
      responseCode: (json['response_code'] as num?)?.toInt(),
      errorMessage: json['error_message'] as String?,
      createdAt: json['created_at'] as String,
      sentAt: json['sent_at'] as String?,
    );

Map<String, dynamic> _$NotificationModelToJson(NotificationModel instance) =>
    <String, dynamic>{
      'id': instance.id,
      'incident_id': instance.incidentId,
      'channel': instance.channel,
      'recipient': instance.recipient,
      'status': instance.status,
      'attempts': instance.attempts,
      'response_code': instance.responseCode,
      'error_message': instance.errorMessage,
      'created_at': instance.createdAt,
      'sent_at': instance.sentAt,
    };
