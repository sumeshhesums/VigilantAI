import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/role_model.dart';

abstract class RoleRemoteDataSource {
  Future<List<RoleModel>> getRoles();
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
}
