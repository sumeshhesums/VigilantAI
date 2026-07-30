import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../core/theme/app_colors.dart';
import '../../../../di/providers.dart';
import '../../domain/entities/camera.dart';
import '../widgets/camera_status_badge.dart';

class CameraDetailPage extends ConsumerStatefulWidget {
  final String cameraId;

  const CameraDetailPage({super.key, required this.cameraId});

  @override
  ConsumerState<CameraDetailPage> createState() => _CameraDetailPageState();
}

class _CameraDetailPageState extends ConsumerState<CameraDetailPage> {
  Camera? _camera;
  bool _isLoading = true;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _loadCamera();
  }

  Future<void> _loadCamera() async {
    setState(() {
      _isLoading = true;
      _errorMessage = null;
    });

    final repo = ref.read(cameraRepositoryProvider);
    final result = await repo.getCameraById(widget.cameraId);
    result.fold(
      (failure) {
        if (mounted) setState(() {
          _errorMessage = failure.message;
          _isLoading = false;
        });
      },
      (camera) {
        if (mounted) setState(() {
          _camera = camera;
          _isLoading = false;
        });
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Camera Details')),
      body: _buildBody(),
    );
  }

  Widget _buildBody() {
    if (_isLoading) {
      return const Center(child: CircularProgressIndicator());
    }
    if (_errorMessage != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error_outline, size: 48, color: AppColors.error),
            const SizedBox(height: 16),
            Text(
              _errorMessage!,
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _loadCamera,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }
    final camera = _camera!;
    return _buildContent(context, camera);
  }

  Widget _buildContent(BuildContext context, Camera camera) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Center(
            child: Icon(
              camera.enabled ? Icons.videocam : Icons.videocam_off,
              size: 64,
              color: camera.enabled ? AppColors.primary : Colors.grey,
            ),
          ),
          const SizedBox(height: 16),
          Center(
            child: CameraStatusBadge(status: camera.status),
          ),
          const SizedBox(height: 24),
          _infoRow(Icons.videocam, 'Name', camera.name),
          if (camera.location != null)
            _infoRow(Icons.location_on, 'Location', camera.location!),
          _infoRow(Icons.link, 'RTSP URL', camera.rtspUrl),
          if (camera.resolution != null)
            _infoRow(Icons.aspect_ratio, 'Resolution', camera.resolution!),
          _infoRow(Icons.speed, 'Status', camera.status),
          _infoRow(
            Icons.toggle_on,
            'Enabled',
            camera.enabled ? 'Yes' : 'No',
          ),
        ],
      ),
    );
  }

  Widget _infoRow(IconData icon, String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        children: [
          Icon(icon, size: 20, color: Colors.grey[600]),
          const SizedBox(width: 12),
          SizedBox(
            width: 80,
            child: Text(
              label,
              style: const TextStyle(
                fontWeight: FontWeight.w600,
                color: Colors.grey,
              ),
            ),
          ),
          Expanded(
            child: Text(value),
          ),
        ],
      ),
    );
  }
}
