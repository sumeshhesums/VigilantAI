import 'package:dartz/dartz.dart';

abstract class Failure {
  final String message;
  final int? statusCode;

  const Failure({required this.message, this.statusCode});

  @override
  String toString() => message;
}

class ServerFailure extends Failure {
  const ServerFailure({super.message = 'Server error', super.statusCode});
}

class AuthFailure extends Failure {
  const AuthFailure({super.message = 'Authentication error', super.statusCode});
}

class NetworkFailure extends Failure {
  const NetworkFailure({super.message = 'No internet connection'});
}

class CacheFailure extends Failure {
  const CacheFailure({super.message = 'Cache error'});
}

class ValidationFailure extends Failure {
  final Map<String, dynamic>? errors;

  const ValidationFailure({
    super.message = 'Validation error',
    this.errors,
  });
}

class NotFoundFailure extends Failure {
  const NotFoundFailure({super.message = 'Resource not found', super.statusCode = 404});
}

class PermissionFailure extends Failure {
  const PermissionFailure({super.message = 'Permission denied', super.statusCode = 403});
}

typedef AsyncResult<T> = Future<Either<Failure, T>>;
