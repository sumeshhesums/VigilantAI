// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'alert_trends_model.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

AlertTrendPointModel _$AlertTrendPointModelFromJson(
        Map<String, dynamic> json) =>
    AlertTrendPointModel(
      timestamp: json['timestamp'] as String,
      critical: (json['critical'] as num).toInt(),
      high: (json['high'] as num).toInt(),
      medium: (json['medium'] as num).toInt(),
      low: (json['low'] as num).toInt(),
    );

Map<String, dynamic> _$AlertTrendPointModelToJson(
        AlertTrendPointModel instance) =>
    <String, dynamic>{
      'timestamp': instance.timestamp,
      'critical': instance.critical,
      'high': instance.high,
      'medium': instance.medium,
      'low': instance.low,
    };

AlertTrendsModel _$AlertTrendsModelFromJson(Map<String, dynamic> json) =>
    AlertTrendsModel(
      interval: json['interval'] as String,
      series: (json['series'] as List<dynamic>)
          .map((e) => AlertTrendPointModel.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$AlertTrendsModelToJson(AlertTrendsModel instance) =>
    <String, dynamic>{
      'interval': instance.interval,
      'series': instance.series,
    };
