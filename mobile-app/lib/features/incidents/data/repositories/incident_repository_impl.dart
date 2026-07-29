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
    int perPage = 20,
  }) async {
    try {
      final model = await _remoteDataSource.getIncidents(page: page, perPage: perPage);
      return Right(PaginatedIncidents(
        incidents: model.incidents.map(_mapToEntity).toList(),
        total: model.total,
        page: model.page,
        perPage: model.perPage,
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
    required String severity,
    required String eventType,
    required double confidence,
    Map<String, dynamic>? boundingBox,
    Map<String, dynamic>? metadata,
  }) async {
    try {
      final request = CreateIncidentRequest(
        cameraId: cameraId,
        severity: severity,
        eventType: eventType,
        confidence: confidence,
        boundingBox: boundingBox,
        metadata: metadata,
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
    required String status,
  }) async {
    try {
      final request = UpdateIncidentRequest(status: status);
      final model = await _remoteDataSource.updateIncident(id, request);
      return Right(_mapToEntity(model));
    } on Failure catch (f) {
      return Left(f);
    } catch (e) {
      return const Left(ServerFailure(message: 'Failed to update incident'));
    }
  }

  Incident _mapToEntity(IncidentModel model) {
    return Incident(
      id: model.id,
      cameraId: model.cameraId,
      timestamp: DateTime.parse(model.timestamp),
      severity: model.severity,
      status: model.status,
      eventType: model.eventType,
      confidence: model.confidence,
      boundingBox: model.boundingBox,
      metadata: model.metadata,
      createdAt: DateTime.parse(model.createdAt),
      updatedAt: model.updatedAt != null ? DateTime.parse(model.updatedAt!) : null,
    );
  }
}
