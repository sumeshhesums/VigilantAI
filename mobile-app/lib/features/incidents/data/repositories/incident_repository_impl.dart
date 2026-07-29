import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../../domain/entities/incident.dart';
import '../../domain/repositories/incident_repository.dart';
import '../datasources/incident_remote_datasource.dart';
import '../models/create_incident_request.dart';
import '../models/incident_model.dart';
import '../models/update_incident_request.dart';

class IncidentRepositoryImpl implements IncidentRepository {
  final IncidentRemoteDataSource _remoteDataSource;

  IncidentRepositoryImpl(this._remoteDataSource);

  @override
  Future<Either<Failure, PaginatedIncidents>> getIncidents({
    int page = 1,
    int pageSize = 20,
  }) async {
    try {
      final model = await _remoteDataSource.getIncidents(page: page, pageSize: pageSize);
      return Right(PaginatedIncidents(
        incidents: model.incidents.map(_mapToEntity).toList(),
        total: model.total,
        page: model.page,
        pageSize: model.pageSize,
      ));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch incidents'));
    }
  }

  @override
  Future<Either<Failure, Incident>> getIncidentById(String id) async {
    try {
      final model = await _remoteDataSource.getIncidentById(id);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to fetch incident'));
    }
  }

  @override
  Future<Either<Failure, Incident>> createIncident({
    required String cameraId,
    required String title,
    String? description,
    required String severity,
    String? status,
  }) async {
    try {
      final request = CreateIncidentRequest(
        cameraId: cameraId,
        title: title,
        description: description,
        severity: severity,
        status: status,
      );
      final model = await _remoteDataSource.createIncident(request);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to create incident'));
    }
  }

  @override
  Future<Either<Failure, Incident>> updateIncident(
    String id, {
    String? title,
    String? description,
    String? severity,
    String? status,
  }) async {
    try {
      final request = UpdateIncidentRequest(
        title: title,
        description: description,
        severity: severity,
        status: status,
      );
      final model = await _remoteDataSource.updateIncident(id, request);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to update incident'));
    }
  }

  @override
  Future<Either<Failure, void>> deleteIncident(String id) async {
    try {
      await _remoteDataSource.deleteIncident(id);
      return const Right(null);
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to delete incident'));
    }
  }

  Incident _mapToEntity(IncidentModel model) {
    return Incident(
      id: model.id,
      cameraInfo: CameraInfo(
        cameraId: model.cameraId,
        cameraName: model.cameraName,
      ),
      title: model.title,
      description: model.description,
      severity: model.severity,
      status: model.status,
      detectedAt: DateTime.parse(model.detectedAt),
      acknowledgedAt: model.acknowledgedAt != null ? DateTime.parse(model.acknowledgedAt!) : null,
      resolvedAt: model.resolvedAt != null ? DateTime.parse(model.resolvedAt!) : null,
      createdAt: DateTime.parse(model.createdAt),
      updatedAt: DateTime.parse(model.updatedAt),
    );
  }
}
