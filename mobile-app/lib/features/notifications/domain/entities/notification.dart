class Notification {
  final String id;
  final String incidentId;
  final String channel;
  final String recipient;
  final String status;
  final int attempts;
  final int? responseCode;
  final String? errorMessage;
  final DateTime createdAt;
  final DateTime? sentAt;

  const Notification({
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

  bool get isUnread => status == 'pending';
}
