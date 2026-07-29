class LiveStats {
  final int activeAlerts;
  final int camerasOnline;
  final int camerasOffline;
  final int detectionsToday;
  final double uptimePercentage;
  final double avgFps;

  const LiveStats({
    required this.activeAlerts,
    required this.camerasOnline,
    required this.camerasOffline,
    required this.detectionsToday,
    required this.uptimePercentage,
    required this.avgFps,
  });
}
