class Notification {
  final String id;
  final String title;
  final String message;
  final String type;
  final bool read;
  final String? incidentId;
  final DateTime createdAt;

  const Notification({
    required this.id,
    required this.title,
    required this.message,
    required this.type,
    required this.read,
    this.incidentId,
    required this.createdAt,
  });

  bool get isUnread => !read;
}
