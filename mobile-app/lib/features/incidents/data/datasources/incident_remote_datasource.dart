import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/create_incident_request.dart';
import '../models/incident_model.dart';
import '../models/update_incident_request.dart';

abstract class IncidentRemoteDataSource {
  Future<PaginatedIncidentsModel> getIncidents({int page = 1, int pageSize = 20});
  Future<IncidentModel> getIncidentById(String id);
  Future<IncidentModel> createIncident(CreateIncidentRequest request);
  Future<IncidentModel> updateIncident(String id, UpdateIncidentRequest request);
  Future<void> deleteIncident(String id);
}

class IncidentRemoteDataSourceImpl implements IncidentRemoteDataSource {
  final ApiClient _client;

  IncidentRemoteDataSourceImpl(this._client);

  @override
  Future<PaginatedIncidentsModel> getIncidents({int page = 1, int pageSize = 20}) async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.incidents,
      queryParameters: {'page': page, 'page_size': pageSize},
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
    final result = await _client.put<Map<String, dynamic>>(
      '${ApiConstants.incidentById}$id',
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => IncidentModel.fromJson(response.data!),
    );
  }

  @override
  Future<void> deleteIncident(String id) async {
    final result = await _client.delete<Map<String, dynamic>>(
      '${ApiConstants.incidentById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => null,
    );
  }
}
