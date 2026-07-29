import 'package:dartz/dartz.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

import '../../../../core/constants/app_constants.dart';
import '../../../../core/errors/failures.dart';
import '../../../../core/network/api_client.dart';
import '../../domain/entities/auth_tokens.dart';
import '../../domain/entities/user.dart';
import '../../domain/repositories/auth_repository.dart';
import '../datasources/auth_remote_datasource.dart';
import '../models/auth_response.dart';
import '../models/login_request.dart';
import '../models/refresh_token_request.dart';
import '../models/register_request.dart';
import '../models/user_model.dart' as user_model;

class AuthRepositoryImpl implements AuthRepository {
  final AuthRemoteDataSource _remoteDataSource;
  final ApiClient _apiClient;
  final FlutterSecureStorage _secureStorage;

  AuthRepositoryImpl(
    this._remoteDataSource,
    this._apiClient,
    this._secureStorage,
  );

  @override
  Future<Either<Failure, AuthTokens>> login(
    String email,
    String password,
  ) async {
    try {
      final request = LoginRequest(email: email, password: password);
      final response = await _remoteDataSource.login(request);
      await _saveTokens(response);
      _apiClient.updateToken(response.accessToken);
      return Right(_mapToAuthTokens(response));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Login failed'));
    }
  }

  @override
  Future<Either<Failure, User>> register({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
  }) async {
    try {
      final request = RegisterRequest(
        email: email,
        password: password,
        firstName: firstName,
        lastName: lastName,
      );
      final response = await _remoteDataSource.register(request);
      return Right(_mapToUser(response));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Registration failed'));
    }
  }

  @override
  Future<Either<Failure, AuthTokens>> refreshToken(String refreshToken) async {
    try {
      final request = RefreshTokenRequest(refreshToken: refreshToken);
      final response = await _remoteDataSource.refreshToken(request);
      await _saveTokens(response);
      _apiClient.updateToken(response.accessToken);
      return Right(_mapToAuthTokens(response));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Token refresh failed'));
    }
  }

  @override
  Future<Either<Failure, User>> getCurrentUser() async {
    try {
      final response = await _remoteDataSource.getCurrentUser();
      return Right(_mapToUser(response));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to get user'));
    }
  }

  @override
  Future<Either<Failure, void>> logout() async {
    try {
      await _secureStorage.delete(key: AppConstants.accessTokenKey);
      await _secureStorage.delete(key: AppConstants.refreshTokenKey);
      await _secureStorage.delete(key: AppConstants.userKey);
      _apiClient.clearToken();
      return const Right(null);
    } catch (e) {
      return const Left(CacheFailure(message: 'Failed to clear session'));
    }
  }

  @override
  Future<Either<Failure, bool>> isAuthenticated() async {
    try {
      final token = await _secureStorage.read(key: AppConstants.accessTokenKey);
      return Right(token != null && token.isNotEmpty);
    } catch (e) {
      return const Right(false);
    }
  }

  Future<void> _saveTokens(AuthResponse response) async {
    await _secureStorage.write(
      key: AppConstants.accessTokenKey,
      value: response.accessToken,
    );
    await _secureStorage.write(
      key: AppConstants.refreshTokenKey,
      value: response.refreshToken,
    );
  }

  AuthTokens _mapToAuthTokens(AuthResponse response) {
    return AuthTokens(
      accessToken: response.accessToken,
      refreshToken: response.refreshToken,
      expiresIn: response.expiresIn,
      tokenType: response.tokenType,
    );
  }

  User _mapToUser(user_model.UserModel model) {
    return User(
      id: model.id,
      email: model.email,
      firstName: model.firstName,
      lastName: model.lastName,
      role: model.role,
      createdAt: DateTime.parse(model.createdAt),
    );
  }
}
