import 'package:dartz/dartz.dart';
import 'package:dio/dio.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

import 'package:vigilantai/core/errors/failures.dart';
import 'package:vigilantai/core/config/app_config.dart';
import 'package:vigilantai/core/network/api_client.dart';
import 'package:vigilantai/core/network/api_interceptors.dart';


class DioClient implements ApiClient {
  late final Dio _dio;
  late final FlutterSecureStorage _secureStorage;

  DioClient() {
    _secureStorage = const FlutterSecureStorage();
    _dio = Dio(
      BaseOptions(
        baseUrl: '${AppConfig.baseUrl}${AppConfig.apiPrefix}',
        connectTimeout: AppConfig.connectTimeout,
        receiveTimeout: AppConfig.receiveTimeout,
        sendTimeout: AppConfig.sendTimeout,
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
        },
      ),
    );

    _dio.interceptors.addAll([
      AuthInterceptor(_secureStorage),
      LoggingInterceptor(),
    ]);
  }

  @override
  Future<Either<Failure, Response<T>>> get<T>(
    String path, {
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
  }) async {
    return _safeCall(() => _dio.get<T>(
          path,
          queryParameters: queryParameters,
          options: options,
          cancelToken: cancelToken,
        ));
  }

  @override
  Future<Either<Failure, Response<T>>> post<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
  }) async {
    return _safeCall(() => _dio.post<T>(
          path,
          data: data,
          queryParameters: queryParameters,
          options: options,
          cancelToken: cancelToken,
        ));
  }

  @override
  Future<Either<Failure, Response<T>>> put<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
  }) async {
    return _safeCall(() => _dio.put<T>(
          path,
          data: data,
          queryParameters: queryParameters,
          options: options,
          cancelToken: cancelToken,
        ));
  }

  @override
  Future<Either<Failure, Response<T>>> patch<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
  }) async {
    return _safeCall(() => _dio.patch<T>(
          path,
          data: data,
          queryParameters: queryParameters,
          options: options,
          cancelToken: cancelToken,
        ));
  }

  @override
  Future<Either<Failure, Response<T>>> delete<T>(
    String path, {
    dynamic data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
  }) async {
    return _safeCall(() => _dio.delete<T>(
          path,
          data: data,
          queryParameters: queryParameters,
          options: options,
          cancelToken: cancelToken,
        ));
  }

  @override
  Future<Either<Failure, Response<T>>> upload<T>(
    String path, {
    required FormData data,
    Options? options,
    CancelToken? cancelToken,
    void Function(int, int)? onSendProgress,
  }) async {
    return _safeCall(() => _dio.post<T>(
          path,
          data: data,
          options: options,
          cancelToken: cancelToken,
          onSendProgress: onSendProgress,
        ));
  }

  @override
  void updateToken(String? accessToken) {
    if (accessToken != null) {
      _dio.options.headers['Authorization'] = 'Bearer $accessToken';
    } else {
      _dio.options.headers.remove('Authorization');
    }
  }

  @override
  void clearToken() {
    _dio.options.headers.remove('Authorization');
  }

  Future<Either<Failure, Response<T>>> _safeCall<T>(
    Future<Response<T>> Function() call,
  ) async {
    try {
      final response = await call();
      return Right(response);
    } on DioException catch (e) {
      return Left(_mapDioError(e));
    } catch (e) {
      return const Left(ServerFailure(message: 'Unexpected error'));
    }
  }

  Failure _mapDioError(DioException error) {
    switch (error.type) {
      case DioExceptionType.connectionTimeout:
      case DioExceptionType.sendTimeout:
      case DioExceptionType.receiveTimeout:
        return const NetworkFailure(message: 'Connection timed out');

      case DioExceptionType.connectionError:
        return const NetworkFailure(message: 'No internet connection');

      case DioExceptionType.badResponse:
        return _mapStatusCodeError(error);

      case DioExceptionType.cancel:
        return const ServerFailure(message: 'Request cancelled');

      default:
        return const ServerFailure(message: 'Unexpected error');
    }
  }

  Failure _mapStatusCodeError(DioException error) {
    final statusCode = error.response?.statusCode;
    final data = error.response?.data;
    final message = data is Map ? data['message'] as String? : null;

    switch (statusCode) {
      case 400:
        return ValidationFailure(
          message: message ?? 'Invalid request',
          errors: data is Map ? data['errors'] as Map<String, dynamic>? : null,
        );
      case 401:
        return AuthFailure(
          message: message ?? 'Unauthorized',
          statusCode: statusCode,
        );
      case 403:
        return PermissionFailure(
          message: message ?? 'Forbidden',
        );
      case 404:
        return NotFoundFailure(
          message: message ?? 'Resource not found',
        );
      case 409:
        return ServerFailure(
          message: message ?? 'Conflict',
          statusCode: statusCode,
        );
      case 422:
        return ValidationFailure(
          message: message ?? 'Validation error',
          errors: data is Map ? data['errors'] as Map<String, dynamic>? : null,
        );
      case 429:
        return ServerFailure(
          message: message ?? 'Too many requests',
          statusCode: statusCode,
        );
      case 500:
      case 502:
      case 503:
        return ServerFailure(
          message: message ?? 'Server error',
          statusCode: statusCode,
        );
      default:
        return ServerFailure(
          message: message ?? 'Unexpected error',
          statusCode: statusCode,
        );
    }
  }
}
