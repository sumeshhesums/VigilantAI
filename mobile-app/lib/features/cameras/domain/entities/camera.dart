class Camera {
  final String id;
  final String name;
  final String location;
  final String status;
  final bool enabled;
  final String? streamUrl;
  final String? rtspUrl;
  final String? model;
  final String? ipAddress;
  final int? port;
  final String? username;
  final int? fps;
  final int? resolutionWidth;
  final int? resolutionHeight;
  final DateTime createdAt;
  final DateTime? updatedAt;

  const Camera({
    required this.id,
    required this.name,
    required this.location,
    required this.status,
    required this.enabled,
    this.streamUrl,
    this.rtspUrl,
    this.model,
    this.ipAddress,
    this.port,
    this.username,
    this.fps,
    this.resolutionWidth,
    this.resolutionHeight,
    required this.createdAt,
    this.updatedAt,
  });

  bool get isOnline => status == 'online';
}
