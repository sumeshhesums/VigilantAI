import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incident.dart';

abstract class IncidentRepository {
  Future<Either<Failure, PaginatedIncidents>> getIncidents({
    int page = 1,
    int perPage = 20,
  });

  Future<Either<Failure, Incident>> getIncidentById(String id);

  Future<Either<Failure, Incident>> createIncident({
    required String cameraId,
    required String severity,
    required String eventType,
    required double confidence,
    Map<String, dynamic>? boundingBox,
    Map<String, dynamic>? metadata,
  });

  Future<Either<Failure, Incident>> updateIncident(
    String id, {
    required String status,
  });
}
