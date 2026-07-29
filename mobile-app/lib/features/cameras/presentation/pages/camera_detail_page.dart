import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../core/theme/app_colors.dart';
import '../../../../di/providers.dart';
import '../../domain/entities/camera.dart';
import '../widgets/camera_status_badge.dart';

class CameraDetailPage extends ConsumerWidget {
  final String cameraId;

  const CameraDetailPage({super.key, required this.cameraId});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final cameraFuture = ref.watch(cameraRepositoryProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Camera Details')),
      body: Center(
        child: FutureBuilder<Camera>(
          future: cameraFuture.getCameraById(cameraId).then(
            (result) => result.fold(
              (failure) => throw Exception(failure.message),
              (camera) => camera,
            ),
          ),
          builder: (context, snapshot) {
            if (snapshot.connectionState == ConnectionState.waiting) {
              return const CircularProgressIndicator();
            }
            if (snapshot.hasError) {
              return Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Icon(Icons.error_outline, size: 48, color: AppColors.error),
                  const SizedBox(height: 16),
                  Text('Error: ${snapshot.error}'),
                ],
              );
            }
            final camera = snapshot.data!;
            return _buildContent(context, camera);
          },
        ),
      ),
    );
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
