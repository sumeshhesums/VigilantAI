import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/evidence.dart';

abstract class EvidenceRepository {
  Future<Either<Failure, List<Evidence>>> getEvidence({int page = 1, int pageSize = 20});
  Future<Either<Failure, List<Evidence>>> getEvidenceByIncident(String incidentId);
  Future<Either<Failure, Evidence>> uploadEvidence({
    required String filePath,
    required String fileName,
    required String incidentId,
  });
  Future<Either<Failure, void>> deleteEvidence(String id);
}
