// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'update_role_request.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

UpdateRoleRequest _$UpdateRoleRequestFromJson(Map<String, dynamic> json) =>
    UpdateRoleRequest(
      name: json['name'] as String?,
      description: json['description'] as String?,
      permissions: (json['permissions'] as List<dynamic>?)
          ?.map((e) => e as String)
          .toList(),
    );

Map<String, dynamic> _$UpdateRoleRequestToJson(UpdateRoleRequest instance) =>
    <String, dynamic>{
      'name': instance.name,
      'description': instance.description,
      'permissions': instance.permissions,
    };
