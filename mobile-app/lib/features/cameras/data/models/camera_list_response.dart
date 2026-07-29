import 'package:json_annotation/json_annotation.dart';

import 'camera_model.dart';

part 'camera_list_response.g.dart';

@JsonSerializable()
class CameraListResponse {
  final List<CameraModel> cameras;
  final int total;
  final int page;
  @JsonKey(name: 'per_page')
  final int perPage;

  const CameraListResponse({
    required this.cameras,
    required this.total,
    required this.page,
    required this.perPage,
  });

  factory CameraListResponse.fromJson(Map<String, dynamic> json) =>
      _$CameraListResponseFromJson(json);
}
