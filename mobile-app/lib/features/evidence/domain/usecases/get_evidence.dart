import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/evidence.dart';
import '../repositories/evidence_repository.dart';

class GetEvidenceUseCase {
  final EvidenceRepository _repository;

  GetEvidenceUseCase(this._repository);

  Future<Either<Failure, List<Evidence>>> execute({int page = 1, int pageSize = 20}) {
    return _repository.getEvidence(page: page, pageSize: pageSize);
  }
}
