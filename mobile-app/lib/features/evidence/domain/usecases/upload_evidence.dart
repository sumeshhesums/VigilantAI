import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/evidence.dart';
import '../repositories/evidence_repository.dart';

class UploadEvidenceUseCase {
  final EvidenceRepository _repository;

  UploadEvidenceUseCase(this._repository);

  Future<Either<Failure, Evidence>> execute({
    required String filePath,
    required String fileName,
    required String incidentId,
  }) {
    return _repository.uploadEvidence(
      filePath: filePath,
      fileName: fileName,
      incidentId: incidentId,
    );
  }
}
