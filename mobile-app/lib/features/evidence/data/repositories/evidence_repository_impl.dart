import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../../domain/entities/evidence.dart';
import '../../domain/repositories/evidence_repository.dart';
import '../datasources/evidence_remote_datasource.dart';
import '../models/evidence_model.dart';

class EvidenceRepositoryImpl implements EvidenceRepository {
  final EvidenceRemoteDataSource _remoteDataSource;

  EvidenceRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, List<Evidence>>> getEvidence({int page = 1, int pageSize = 20}) async {
    try {
      final response = await _remoteDataSource.getEvidence(page: page, pageSize: pageSize);
      final list = (response['evidence'] as List)
          .map((e) => _mapToEntity(EvidenceModel.fromJson(e as Map<String, dynamic>)))
          .toList();
      return Right(list);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch evidence'));
    }
  }

  @override
  Future<Either<Failure, List<Evidence>>> getEvidenceByIncident(String incidentId) async {
    try {
      final response = await _remoteDataSource.getEvidenceByIncident(incidentId);
      final list = (response['evidence'] as List)
          .map((e) => _mapToEntity(EvidenceModel.fromJson(e as Map<String, dynamic>)))
          .toList();
      return Right(list);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch evidence for incident'));
    }
  }

  @override
  Future<Either<Failure, Evidence>> uploadEvidence({
    required String filePath,
    required String fileName,
    required String incidentId,
  }) async {
    try {
      final model = await _remoteDataSource.uploadEvidence(
        filePath: filePath,
        fileName: fileName,
        incidentId: incidentId,
      );
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to upload evidence'));
    }
  }

  @override
  Future<Either<Failure, void>> deleteEvidence(String id) async {
    try {
      await _remoteDataSource.deleteEvidence(id);
      return const Right(null);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to delete evidence'));
    }
  }

  Evidence _mapToEntity(EvidenceModel model) {
    return Evidence(
      id: model.id,
      incidentId: model.incidentId,
      fileName: model.fileName,
      fileType: model.fileType,
      fileSize: model.fileSize,
      fileUrl: model.fileUrl,
      thumbnailUrl: model.thumbnailUrl,
      uploadedBy: model.uploadedBy,
      uploadedAt: DateTime.parse(model.uploadedAt),
      createdAt: DateTime.parse(model.createdAt),
    );
  }
}
