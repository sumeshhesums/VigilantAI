import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../repositories/evidence_repository.dart';

class DeleteEvidenceUseCase {
  final EvidenceRepository _repository;

  DeleteEvidenceUseCase(this._repository);

  Future<Either<Failure, void>> execute(String id) {
    return _repository.deleteEvidence(id);
  }
}
