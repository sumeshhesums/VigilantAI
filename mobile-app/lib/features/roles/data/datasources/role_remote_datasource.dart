import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/create_role_request.dart';
import '../models/role_model.dart';
import '../models/update_role_request.dart';

abstract class RoleRemoteDataSource {
  Future<List<RoleModel>> getRoles();
  Future<RoleModel> getRoleById(String id);
  Future<RoleModel> createRole(CreateRoleRequest request);
  Future<RoleModel> updateRole(String id, UpdateRoleRequest request);
  Future<void> deleteRole(String id);
}

class RoleRemoteDataSourceImpl implements RoleRemoteDataSource {
  final ApiClient _client;

  RoleRemoteDataSourceImpl(this._client);

  @override
  Future<List<RoleModel>> getRoles() async {
    final result = await _client.get<List<dynamic>>(
      ApiConstants.roles,
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data!
          .map((e) => RoleModel.fromJson(e as Map<String, dynamic>))
          .toList(),
    );
  }

  @override
  Future<RoleModel> getRoleById(String id) async {
    final result = await _client.get<Map<String, dynamic>>(
      '${ApiConstants.roleById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => RoleModel.fromJson(response.data!),
    );
  }

  @override
  Future<RoleModel> createRole(CreateRoleRequest request) async {
    final result = await _client.post<Map<String, dynamic>>(
      ApiConstants.roles,
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => RoleModel.fromJson(response.data!),
    );
  }

  @override
  Future<RoleModel> updateRole(String id, UpdateRoleRequest request) async {
    final result = await _client.put<Map<String, dynamic>>(
      '${ApiConstants.roleById}$id',
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => RoleModel.fromJson(response.data!),
    );
  }

  @override
  Future<void> deleteRole(String id) async {
    final result = await _client.delete<void>(
      '${ApiConstants.roleById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data,
    );
  }
}
