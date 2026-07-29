import 'package:json_annotation/json_annotation.dart';

import 'camera_model.dart';

part 'camera_list_response.g.dart';

@JsonSerializable()
class CameraListResponse {
  final List<CameraModel> cameras;
  final int total;
  final int page;
  @JsonKey(name: 'page_size')
  final int pageSize;

  const CameraListResponse({
    required this.cameras,
    required this.total,
    required this.page,
    required this.pageSize,
  });

  factory CameraListResponse.fromJson(Map<String, dynamic> json) =>
      _$CameraListResponseFromJson(json);
}
