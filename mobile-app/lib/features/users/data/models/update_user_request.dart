import 'package:json_annotation/json_annotation.dart';

part 'update_user_request.g.dart';

@JsonSerializable()
class UpdateUserRequest {
  final String? email;
  @JsonKey(name: 'first_name')
  final String? firstName;
  @JsonKey(name: 'last_name')
  final String? lastName;
  final String? role;
  final bool? enabled;

  const UpdateUserRequest({
    this.email,
    this.firstName,
    this.lastName,
    this.role,
    this.enabled,
  });

  Map<String, dynamic> toJson() => _$UpdateUserRequestToJson(this);
}
