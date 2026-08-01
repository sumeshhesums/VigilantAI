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
    return RefreshIndicator(
      onRefresh: _loadCamera,
      child: _buildContent(context, camera),
    );
  }

  Widget _buildContent(BuildContext context, Camera camera) {
    return SingleChildScrollView(
      physics: const AlwaysScrollableScrollPhysics(),
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
          const SizedBox(height: 8),
          Center(
            child: Text(
              camera.isOnline ? 'Camera is Online' : 'Camera is Offline',
              style: TextStyle(
                fontSize: 13,
                color: camera.isOnline ? Colors.green : Colors.grey,
              ),
            ),
          ),
          const SizedBox(height: 24),
          _infoRow(Icons.videocam, 'Name', camera.name),
          if (camera.location != null)
            _infoRow(Icons.location_on, 'Location', camera.location!),
          _infoRow(Icons.link, 'RTSP URL', camera.rtspUrl),
          if (camera.fps != null)
            _infoRow(Icons.speed, 'FPS', '${camera.fps}'),
          if (camera.resolution != null)
            _infoRow(Icons.aspect_ratio, 'Resolution', camera.resolution!),
          _infoRow(Icons.visibility, 'Status', camera.status),
          if (camera.lastSeen != null)
            _infoRow(
              Icons.access_time,
              'Last Seen',
              _formatLastSeen(camera.lastSeen!),
            ),
          _infoRow(
            Icons.toggle_on,
            'Enabled',
            camera.enabled ? 'Yes' : 'No',
          ),
          const SizedBox(height: 24),
          _buildStreamHealthCard(camera),
        ],
      ),
    );
  }

  Widget _buildStreamHealthCard(Camera camera) {
    final isHealthy = camera.isOnline;
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  isHealthy ? Icons.check_circle : Icons.warning,
                  color: isHealthy ? Colors.green : Colors.orange,
                  size: 20,
                ),
                const SizedBox(width: 8),
                Text(
                  'Stream Health',
                  style: const TextStyle(
                    fontWeight: FontWeight.w600,
                    fontSize: 15,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            _healthRow(
              Icons.sensors,
              'Connection',
              isHealthy ? 'Connected' : 'Disconnected',
              isHealthy ? Colors.green : Colors.red,
            ),
            const SizedBox(height: 8),
            _healthRow(
              Icons.cloud_done,
              'Backend Sync',
              'Via REST API',
              AppColors.primary,
            ),
            if (camera.lastSeen != null) ...[
              const SizedBox(height: 8),
              _healthRow(
                Icons.update,
                'Last Update',
                _formatLastSeen(camera.lastSeen!),
                Colors.grey,
              ),
            ],
            const Divider(height: 24),
            Text(
              'Live streaming is not yet implemented. '
              'The status shown above is provided by the backend '
              'based on the Camera Gateway connection state.',
              style: TextStyle(
                fontSize: 12,
                color: Colors.grey[600],
                height: 1.4,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _healthRow(IconData icon, String label, String value, Color color) {
    return Row(
      children: [
        Icon(icon, size: 16, color: color),
        const SizedBox(width: 8),
        Text(
          label,
          style: const TextStyle(fontSize: 13, color: Colors.grey),
        ),
        const Spacer(),
        Text(
          value,
          style: TextStyle(fontSize: 13, fontWeight: FontWeight.w500, color: color),
        ),
      ],
    );
  }

  String _formatLastSeen(DateTime lastSeen) {
    final diff = DateTime.now().difference(lastSeen);
    if (diff.inSeconds < 60) return '${diff.inSeconds}s ago';
    if (diff.inMinutes < 60) return '${diff.inMinutes}m ago';
    if (diff.inHours < 24) return '${diff.inHours}h ago';
    return '${diff.inDays}d ago';
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
