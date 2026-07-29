import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/auth_response.dart';
import '../models/login_request.dart';
import '../models/refresh_token_request.dart';
import '../models/register_request.dart';
import '../models/user_model.dart';

abstract class AuthRemoteDataSource {
  Future<AuthResponse> login(LoginRequest request);
  Future<UserModel> register(RegisterRequest request);
  Future<AuthResponse> refreshToken(RefreshTokenRequest request);
  Future<UserModel> getCurrentUser();
}

class AuthRemoteDataSourceImpl implements AuthRemoteDataSource {
  final ApiClient _client;

  AuthRemoteDataSourceImpl(this._client);

  @override
  Future<AuthResponse> login(LoginRequest request) async {
    final result = await _client.post<Map<String, dynamic>>(
      ApiConstants.login,
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => AuthResponse.fromJson(response.data!),
    );
  }

  @override
  Future<UserModel> register(RegisterRequest request) async {
    final result = await _client.post<Map<String, dynamic>>(
      ApiConstants.register,
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => UserModel.fromJson(response.data!),
    );
  }

  @override
  Future<AuthResponse> refreshToken(RefreshTokenRequest request) async {
    final result = await _client.post<Map<String, dynamic>>(
      ApiConstants.refreshToken,
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => AuthResponse.fromJson(response.data!),
    );
  }

  @override
  Future<UserModel> getCurrentUser() async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.me,
    );
    return result.fold(
      (failure) => throw failure,
      (response) => UserModel.fromJson(response.data!),
    );
  }
}
