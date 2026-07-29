class Camera {
  final String id;
  final String name;
  final String? location;
  final String status;
  final bool enabled;
  final String rtspUrl;
  final int? fps;
  final String? resolution;
  final DateTime? lastSeen;
  final DateTime createdAt;
  final DateTime? updatedAt;

  const Camera({
    required this.id,
    required this.name,
    this.location,
    required this.status,
    required this.enabled,
    required this.rtspUrl,
    this.fps,
    this.resolution,
    this.lastSeen,
    required this.createdAt,
    this.updatedAt,
  });

  bool get isOnline => status == 'online';
}
