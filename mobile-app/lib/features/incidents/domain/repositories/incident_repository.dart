import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/incident.dart';

abstract class IncidentRepository {
  Future<Either<Failure, PaginatedIncidents>> getIncidents({
    int page = 1,
    int pageSize = 20,
  });

  Future<Either<Failure, Incident>> getIncidentById(String id);

  Future<Either<Failure, Incident>> createIncident({
    required String cameraId,
    required String title,
    String? description,
    required String severity,
    String? status,
  });

  Future<Either<Failure, Incident>> updateIncident(
    String id, {
    String? title,
    String? description,
    String? severity,
    String? status,
  });

  Future<Either<Failure, void>> deleteIncident(String id);
}
