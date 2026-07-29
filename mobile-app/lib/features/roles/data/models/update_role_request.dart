import 'package:json_annotation/json_annotation.dart';

part 'update_role_request.g.dart';

@JsonSerializable()
class UpdateRoleRequest {
  final String? name;
  final String? description;
  final List<String>? permissions;

  const UpdateRoleRequest({
    this.name,
    this.description,
    this.permissions,
  });

  Map<String, dynamic> toJson() => _$UpdateRoleRequestToJson(this);
}
