class Incident {
  final String id;
  final String cameraId;
  final DateTime timestamp;
  final String severity;
  final String status;
  final String eventType;
  final double confidence;
  final Map<String, dynamic>? boundingBox;
  final Map<String, dynamic>? metadata;
  final DateTime createdAt;
  final DateTime? updatedAt;

  const Incident({
    required this.id,
    required this.cameraId,
    required this.timestamp,
    required this.severity,
    required this.status,
    required this.eventType,
    required this.confidence,
    this.boundingBox,
    this.metadata,
    required this.createdAt,
    this.updatedAt,
  });

  bool get isOpen => status == 'open';
  bool get isAcknowledged => status == 'acknowledged';
  bool get isResolved => status == 'resolved';
  bool get isFalsePositive => status == 'false_positive';
}

class PaginatedIncidents {
  final List<Incident> incidents;
  final int total;
  final int page;
  final int perPage;

  const PaginatedIncidents({
    required this.incidents,
    required this.total,
    required this.page,
    required this.perPage,
  });
}
