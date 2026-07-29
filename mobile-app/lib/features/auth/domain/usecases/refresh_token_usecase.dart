import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../repositories/auth_repository.dart';

class RefreshTokenUseCase {
  final AuthRepository _repository;

  RefreshTokenUseCase(this._repository);

  Future<Either<Failure, void>> execute(String refreshToken) {
    return _repository.refreshToken(refreshToken).then((value) => value.map((_) => null));
  }
}
