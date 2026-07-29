import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/evidence.dart';

abstract class EvidenceRepository {
  Future<Either<Failure, List<Evidence>>> getEvidenceByIncident(
    String incidentId, {
    int page = 1,
    int perPage = 20,
  });
  Future<Either<Failure, Evidence>> uploadEvidence({
    required String filePath,
    required String fileName,
    required String incidentId,
  });
  Future<Either<Failure, void>> deleteEvidence(String id);
}
