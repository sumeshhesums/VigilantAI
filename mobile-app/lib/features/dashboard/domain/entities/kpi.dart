class Kpi {
  final int activeCameras;
  final int onlineCameras;
  final int offlineCameras;
  final int totalDetections24h;
  final int criticalAlerts;
  final int openIncidents;
  final double avgResponseTimeSeconds;
  final double slaCompliancePercent;
  final String detectionTrend;

  const Kpi({
    required this.activeCameras,
    required this.onlineCameras,
    required this.offlineCameras,
    required this.totalDetections24h,
    required this.criticalAlerts,
    required this.openIncidents,
    required this.avgResponseTimeSeconds,
    required this.slaCompliancePercent,
    required this.detectionTrend,
  });
}
