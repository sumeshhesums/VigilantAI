import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/create_user_request.dart';
import '../models/update_user_request.dart';
import '../models/user_model.dart';

abstract class UserRemoteDataSource {
  Future<Map<String, dynamic>> getUsers({int page = 1, int pageSize = 20});
  Future<UserModel> getUserById(String id);
  Future<UserModel> createUser(CreateUserRequest request);
  Future<UserModel> updateUser(String id, UpdateUserRequest request);
  Future<void> deleteUser(String id);
}

class UserRemoteDataSourceImpl implements UserRemoteDataSource {
  final ApiClient _client;

  UserRemoteDataSourceImpl(this._client);

  @override
  Future<Map<String, dynamic>> getUsers({int page = 1, int pageSize = 20}) async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.users,
      queryParameters: {'page': page, 'page_size': pageSize},
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data!,
    );
  }

  @override
  Future<UserModel> getUserById(String id) async {
    final result = await _client.get<Map<String, dynamic>>(
      '${ApiConstants.userById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => UserModel.fromJson(response.data!),
    );
  }

  @override
  Future<UserModel> createUser(CreateUserRequest request) async {
    final result = await _client.post<Map<String, dynamic>>(
      ApiConstants.users,
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => UserModel.fromJson(response.data!),
    );
  }

  @override
  Future<UserModel> updateUser(String id, UpdateUserRequest request) async {
    final result = await _client.put<Map<String, dynamic>>(
      '${ApiConstants.userById}$id',
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => UserModel.fromJson(response.data!),
    );
  }

  @override
  Future<void> deleteUser(String id) async {
    final result = await _client.delete<void>(
      '${ApiConstants.userById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data,
    );
  }
}
