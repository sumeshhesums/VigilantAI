import 'package:json_annotation/json_annotation.dart';

part 'incidents_summary_model.g.dart';

@JsonSerializable()
class IncidentSummaryItemModel {
  final String status;
  final int count;
  final double percentage;

  const IncidentSummaryItemModel({
    required this.status,
    required this.count,
    required this.percentage,
  });

  factory IncidentSummaryItemModel.fromJson(Map<String, dynamic> json) =>
      _$IncidentSummaryItemModelFromJson(json);
}

@JsonSerializable()
class IncidentsSummaryModel {
  final int total;
  @JsonKey(name: 'by_status')
  final List<IncidentSummaryItemModel> byStatus;
  @JsonKey(name: 'by_severity')
  final List<IncidentSummaryItemModel> bySeverity;

  const IncidentsSummaryModel({
    required this.total,
    required this.byStatus,
    required this.bySeverity,
  });

  factory IncidentsSummaryModel.fromJson(Map<String, dynamic> json) =>
      _$IncidentsSummaryModelFromJson(json);
}
