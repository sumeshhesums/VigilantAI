class IncidentSummaryItem {
  final String status;
  final int count;
  final double percentage;

  const IncidentSummaryItem({
    required this.status,
    required this.count,
    required this.percentage,
  });
}

class IncidentsSummary {
  final int total;
  final List<IncidentSummaryItem> byStatus;
  final List<IncidentSummaryItem> bySeverity;

  const IncidentsSummary({
    required this.total,
    required this.byStatus,
    required this.bySeverity,
  });
}
