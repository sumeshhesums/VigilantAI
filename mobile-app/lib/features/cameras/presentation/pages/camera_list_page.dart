import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/camera_provider.dart';
import '../widgets/camera_card.dart';

class CameraListPage extends ConsumerWidget {
  const CameraListPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = ref.watch(cameraProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Cameras'),
        actions: [
          if (provider.isLoading)
            const Center(child: SizedBox(width: 24, height: 24, child: CircularProgressIndicator(strokeWidth: 2))),
        ],
      ),
      body: _buildBody(provider),
    );
  }

  Widget _buildBody(CameraNotifier provider) {
    switch (provider.status) {
      case CameraLoadStatus.initial:
        return const Center(child: Text('Press refresh to load cameras'));
      case CameraLoadStatus.loading:
        return const Center(child: CircularProgressIndicator());
      case CameraLoadStatus.error:
        return Center(child: Text('Error: ${provider.errorMessage}'));
      case CameraLoadStatus.loaded:
        if (provider.cameras.isEmpty) {
          return const Center(child: Text('No cameras found'));
        }
        return RefreshIndicator(
          onRefresh: provider.loadCameras,
          child: ListView.builder(
            itemCount: provider.cameras.length,
            itemBuilder: (context, index) {
              final camera = provider.cameras[index];
              return CameraCard(
                camera: camera,
                onTap: () => _navigateToDetail(context, camera.id),
              );
            },
          ),
        );
    }
  }

  void _navigateToDetail(BuildContext context, String cameraId) {
    Navigator.of(context).pushNamed('/cameras/$cameraId');
  }
}
