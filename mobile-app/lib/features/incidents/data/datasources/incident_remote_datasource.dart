import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/create_incident_request.dart';
import '../models/incident_model.dart';
import '../models/update_incident_request.dart';

abstract class IncidentRemoteDataSource {
  Future<PaginatedIncidentsModel> getIncidents({int page = 1, int perPage = 20});
  Future<IncidentModel> getIncidentById(String id);
  Future<IncidentModel> createIncident(CreateIncidentRequest request);
  Future<IncidentModel> updateIncident(String id, UpdateIncidentRequest request);
}

class IncidentRemoteDataSourceImpl implements IncidentRemoteDataSource {
  final ApiClient _client;

  IncidentRemoteDataSourceImpl(this._client);

  @override
  Future<PaginatedIncidentsModel> getIncidents({int page = 1, int perPage = 20}) async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.incidents,
      queryParameters: {'page': page, 'per_page': perPage},
    );
    return result.fold(
      (failure) => throw failure,
      (response) => PaginatedIncidentsModel.fromJson(response.data!),
    );
  }

  @override
  Future<IncidentModel> getIncidentById(String id) async {
    final result = await _client.get<Map<String, dynamic>>(
      '${ApiConstants.incidentById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => IncidentModel.fromJson(response.data!),
    );
  }

  @override
  Future<IncidentModel> createIncident(CreateIncidentRequest request) async {
    final result = await _client.post<Map<String, dynamic>>(
      ApiConstants.incidents,
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => IncidentModel.fromJson(response.data!),
    );
  }

  @override
  Future<IncidentModel> updateIncident(String id, UpdateIncidentRequest request) async {
    final result = await _client.patch<Map<String, dynamic>>(
      '${ApiConstants.incidentById}$id',
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => IncidentModel.fromJson(response.data!),
    );
  }
}
