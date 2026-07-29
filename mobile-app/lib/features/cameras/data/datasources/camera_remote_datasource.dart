import '../../../../core/constants/api_constants.dart';
import '../../../../core/network/api_client.dart';
import '../models/camera_list_response.dart';
import '../models/camera_model.dart';
import '../models/create_camera_request.dart';
import '../models/update_camera_request.dart';

abstract class CameraRemoteDataSource {
  Future<CameraListResponse> getCameras({int page = 1, int perPage = 20});
  Future<CameraModel> getCameraById(String id);
  Future<CameraModel> createCamera(CreateCameraRequest request);
  Future<CameraModel> updateCamera(String id, UpdateCameraRequest request);
  Future<void> deleteCamera(String id);
}

class CameraRemoteDataSourceImpl implements CameraRemoteDataSource {
  final ApiClient _client;

  CameraRemoteDataSourceImpl(this._client);

  @override
  Future<CameraListResponse> getCameras({int page = 1, int perPage = 20}) async {
    final result = await _client.get<Map<String, dynamic>>(
      ApiConstants.cameras,
      queryParameters: {'page': page, 'per_page': perPage},
    );
    return result.fold(
      (failure) => throw failure,
      (response) => CameraListResponse.fromJson(response.data!),
    );
  }

  @override
  Future<CameraModel> getCameraById(String id) async {
    final result = await _client.get<Map<String, dynamic>>(
      '${ApiConstants.cameraById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => CameraModel.fromJson(response.data!),
    );
  }

  @override
  Future<CameraModel> createCamera(CreateCameraRequest request) async {
    final result = await _client.post<Map<String, dynamic>>(
      ApiConstants.cameras,
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => CameraModel.fromJson(response.data!),
    );
  }

  @override
  Future<CameraModel> updateCamera(String id, UpdateCameraRequest request) async {
    final result = await _client.patch<Map<String, dynamic>>(
      '${ApiConstants.cameraById}$id',
      data: request.toJson(),
    );
    return result.fold(
      (failure) => throw failure,
      (response) => CameraModel.fromJson(response.data!),
    );
  }

  @override
  Future<void> deleteCamera(String id) async {
    final result = await _client.delete<Map<String, dynamic>>(
      '${ApiConstants.cameraById}$id',
    );
    return result.fold(
      (failure) => throw failure,
      (response) => null,
    );
  }
}
