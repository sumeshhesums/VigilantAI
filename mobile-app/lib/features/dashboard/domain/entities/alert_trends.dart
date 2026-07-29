class AlertTrendPoint {
  final String timestamp;
  final int critical;
  final int high;
  final int medium;
  final int low;

  const AlertTrendPoint({
    required this.timestamp,
    required this.critical,
    required this.high,
    required this.medium,
    required this.low,
  });
}

class AlertTrends {
  final String interval;
  final List<AlertTrendPoint> series;

  const AlertTrends({required this.interval, required this.series});
}
