import 'package:json_annotation/json_annotation.dart';

part 'create_role_request.g.dart';

@JsonSerializable()
class CreateRoleRequest {
  final String name;
  final String? description;
  final List<String> permissions;

  const CreateRoleRequest({
    required this.name,
    this.description,
    required this.permissions,
  });

  Map<String, dynamic> toJson() => _$CreateRoleRequestToJson(this);
}
