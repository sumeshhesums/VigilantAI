import 'package:json_annotation/json_annotation.dart';

part 'notification_model.g.dart';

@JsonSerializable()
class NotificationModel {
  final String id;
  @JsonKey(name: 'incident_id')
  final String incidentId;
  final String channel;
  final String recipient;
  final String status;
  final int attempts;
  @JsonKey(name: 'response_code')
  final int? responseCode;
  @JsonKey(name: 'error_message')
  final String? errorMessage;
  @JsonKey(name: 'created_at')
  final String createdAt;
  @JsonKey(name: 'sent_at')
  final String? sentAt;

  const NotificationModel({
    required this.id,
    required this.incidentId,
    required this.channel,
    required this.recipient,
    required this.status,
    required this.attempts,
    this.responseCode,
    this.errorMessage,
    required this.createdAt,
    this.sentAt,
  });

  factory NotificationModel.fromJson(Map<String, dynamic> json) =>
      _$NotificationModelFromJson(json);
}
