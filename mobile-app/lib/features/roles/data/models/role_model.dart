import 'package:json_annotation/json_annotation.dart';

part 'role_model.g.dart';

@JsonSerializable()
class RoleModel {
  final String id;
  final String name;
  final String? description;

  const RoleModel({
    required this.id,
    required this.name,
    this.description,
  });

  factory RoleModel.fromJson(Map<String, dynamic> json) =>
      _$RoleModelFromJson(json);
}
