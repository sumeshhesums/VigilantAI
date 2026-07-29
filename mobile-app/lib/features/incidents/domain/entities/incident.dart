class CameraInfo {
  final String cameraId;
  final String cameraName;

  const CameraInfo({
    required this.cameraId,
    required this.cameraName,
  });
}

class Incident {
  final String id;
  final CameraInfo cameraInfo;
  final String title;
  final String description;
  final String severity;
  final String status;
  final DateTime detectedAt;
  final DateTime? acknowledgedAt;
  final DateTime? resolvedAt;
  final DateTime createdAt;
  final DateTime updatedAt;

  const Incident({
    required this.id,
    required this.cameraInfo,
    required this.title,
    required this.description,
    required this.severity,
    required this.status,
    required this.detectedAt,
    this.acknowledgedAt,
    this.resolvedAt,
    required this.createdAt,
    required this.updatedAt,
  });
}

class PaginatedIncidents {
  final List<Incident> incidents;
  final int total;
  final int page;
  final int pageSize;

  const PaginatedIncidents({
    required this.incidents,
    required this.total,
    required this.page,
    required this.pageSize,
  });
}
