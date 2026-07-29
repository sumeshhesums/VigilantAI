import 'package:flutter/material.dart';

import '../../domain/entities/camera.dart';
import '../widgets/camera_status_badge.dart';

class CameraDetailPage extends StatelessWidget {
  final Camera camera;

  const CameraDetailPage({super.key, required this.camera});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(camera.name),
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Center(
              child: Icon(
                camera.enabled ? Icons.videocam : Icons.videocam_off,
                size: 64,
                color: camera.enabled ? Colors.blue : Colors.grey,
              ),
            ),
            const SizedBox(height: 16),
            Center(
              child: CameraStatusBadge(status: camera.status),
            ),
            const SizedBox(height: 24),
            _buildInfoRow('Location', camera.location),
            if (camera.ipAddress != null) _buildInfoRow('IP Address', camera.ipAddress!),
            if (camera.port != null) _buildInfoRow('Port', camera.port.toString()),
            if (camera.model != null) _buildInfoRow('Model', camera.model!),
            if (camera.fps != null) _buildInfoRow('FPS', camera.fps.toString()),
            if (camera.resolutionWidth != null && camera.resolutionHeight != null)
              _buildInfoRow('Resolution', '${camera.resolutionWidth}x${camera.resolutionHeight}'),
            if (camera.streamUrl != null) _buildInfoRow('Stream URL', camera.streamUrl!),
            if (camera.rtspUrl != null) _buildInfoRow('RTSP URL', camera.rtspUrl!),
            _buildInfoRow('Created', _formatDate(camera.createdAt)),
            if (camera.updatedAt != null) _buildInfoRow('Updated', _formatDate(camera.updatedAt!)),
          ],
        ),
      ),
    );
  }

  Widget _buildInfoRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 120,
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

  String _formatDate(DateTime date) {
    return '${date.year}-${date.month.toString().padLeft(2, '0')}-${date.day.toString().padLeft(2, '0')} '
        '${date.hour.toString().padLeft(2, '0')}:${date.minute.toString().padLeft(2, '0')}';
  }
}
