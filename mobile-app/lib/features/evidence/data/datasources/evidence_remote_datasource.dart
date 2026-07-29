import 'package:dio/dio.dart';

import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/evidence_model.dart';

abstract class EvidenceRemoteDataSource {
  Future<Map<String, dynamic>> getEvidence({int page = 1, int pageSize = 20});
  Future<Map<String, dynamic>> getEvidenceByIncident(String incidentId);
  Future<EvidenceModel> uploadEvidence({
    required String filePath,
    required String fileName,
    required String incidentId,
  });
  Future<void> deleteEvidence(String id);
}

class EvidenceRemoteDataSourceImpl implements EvidenceRemoteDataSource {
  final ApiClient _client;

  EvidenceRemoteDataSourceImpl(this._client);

  @override
  Future<Map<String, dynamic>> getEvidence({int page = 1, int pageSize = 20}) async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.evidence,
      queryParameters: {'page': page, 'page_size': pageSize},
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data!,
    );
  }

  @override
  Future<Map<String, dynamic>> getEvidenceByIncident(String incidentId) async {
    final result = await _client.get<Map<String, dynamic>>(
      '${ApiConstants.evidenceByIncident}$incidentId',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data!,
    );
  }

  @override
  Future<EvidenceModel> uploadEvidence({
    required String filePath,
    required String fileName,
    required String incidentId,
  }) async {
    final formData = FormData.fromMap({
      'file': await MultipartFile.fromFile(filePath, filename: fileName),
      'incident_id': incidentId,
    });
    final result = await _client.upload<Map<String, dynamic>>(
      ApiConstants.evidenceUpload,
      data: formData,
    );
    return result.fold(
      (failure) => throw failure,
      (response) => EvidenceModel.fromJson(response.data!),
    );
  }

  @override
  Future<void> deleteEvidence(String id) async {
    final result = await _client.delete<void>(
      '${ApiConstants.evidenceById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => response.data,
    );
  }
}
