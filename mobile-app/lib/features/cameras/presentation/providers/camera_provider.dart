import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../domain/entities/camera.dart';
import '../../domain/usecases/get_cameras_usecase.dart';

final cameraProvider = ChangeNotifierProvider<CameraNotifier>((ref) {
  return CameraNotifier();
});

enum CameraLoadStatus { initial, loading, loaded, error }

class CameraNotifier extends ChangeNotifier {
  CameraLoadStatus _status = CameraLoadStatus.initial;
  List<Camera> _cameras = [];
  String? _errorMessage;

  CameraLoadStatus get status => _status;
  List<Camera> get cameras => _cameras;
  String? get errorMessage => _errorMessage;
  bool get isLoading => _status == CameraLoadStatus.loading;

  GetCamerasUseCase? _getCamerasUseCase;

  void setGetCamerasUseCase(GetCamerasUseCase useCase) {
    _getCamerasUseCase = useCase;
  }

  Future<void> loadCameras() async {
    if (_getCamerasUseCase == null) return;

    _status = CameraLoadStatus.loading;
    notifyListeners();

    final result = await _getCamerasUseCase!.execute();
    result.fold(
      (failure) {
        _status = CameraLoadStatus.error;
        _errorMessage = failure.message;
        notifyListeners();
      },
      (cameras) {
        _status = CameraLoadStatus.loaded;
        _cameras = cameras;
        notifyListeners();
      },
    );
  }
}
