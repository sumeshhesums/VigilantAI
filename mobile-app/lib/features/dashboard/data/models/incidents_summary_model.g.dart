// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'incidents_summary_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

IncidentSummaryItemModel _$IncidentSummaryItemModelFromJson(
        Map<String, dynamic> json) =>
    IncidentSummaryItemModel(
      status: json['status'] as String,
      count: (json['count'] as num).toInt(),
      percentage: (json['percentage'] as num).toDouble(),
    );

Map<String, dynamic> _$IncidentSummaryItemModelToJson(
        IncidentSummaryItemModel instance) =>
    <String, dynamic>{
      'status': instance.status,
      'count': instance.count,
      'percentage': instance.percentage,
    };

IncidentsSummaryModel _$IncidentsSummaryModelFromJson(
        Map<String, dynamic> json) =>
    IncidentsSummaryModel(
      total: (json['total'] as num).toInt(),
      byStatus: (json['by_status'] as List<dynamic>)
          .map((e) =>
              IncidentSummaryItemModel.fromJson(e as Map<String, dynamic>))
          .toList(),
      bySeverity: (json['by_severity'] as List<dynamic>)
          .map((e) =>
              IncidentSummaryItemModel.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$IncidentsSummaryModelToJson(
        IncidentsSummaryModel instance) =>
    <String, dynamic>{
      'total': instance.total,
      'by_status': instance.byStatus,
      'by_severity': instance.bySeverity,
    };
