import 'package:json_annotation/json_annotation.dart';

part 'alert_trends_model.g.dart';

@JsonSerializable()
class AlertTrendPointModel {
  final String timestamp;
  final int critical;
  final int high;
  final int medium;
  final int low;

  const AlertTrendPointModel({
    required this.timestamp,
    required this.critical,
    required this.high,
    required this.medium,
    required this.low,
  });

  factory AlertTrendPointModel.fromJson(Map<String, dynamic> json) =>
      _$AlertTrendPointModelFromJson(json);
}

@JsonSerializable()
class AlertTrendsModel {
  final String interval;
  final List<AlertTrendPointModel> series;

  const AlertTrendsModel({required this.interval, required this.series});

  factory AlertTrendsModel.fromJson(Map<String, dynamic> json) =>
      _$AlertTrendsModelFromJson(json);
}
