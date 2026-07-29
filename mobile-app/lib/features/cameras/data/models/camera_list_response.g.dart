// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'camera_list_response.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

CameraListResponse _$CameraListResponseFromJson(Map<String, dynamic> json) =>
    CameraListResponse(
      cameras: (json['cameras'] as List<dynamic>)
          .map((e) => CameraModel.fromJson(e as Map<String, dynamic>))
          .toList(),
      total: (json['total'] as num).toInt(),
      page: (json['page'] as num).toInt(),
      pageSize: (json['page_size'] as num).toInt(),
    );

Map<String, dynamic> _$CameraListResponseToJson(CameraListResponse instance) =>
    <String, dynamic>{
      'cameras': instance.cameras,
      'total': instance.total,
      'page': instance.page,
      'page_size': instance.pageSize,
    };
