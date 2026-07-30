import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../core/theme/app_colors.dart';
import '../providers/camera_provider.dart';
import '../widgets/camera_card.dart';

class CameraListPage extends ConsumerStatefulWidget {
  const CameraListPage({super.key});

  @override
  ConsumerState<CameraListPage> createState() => _CameraListPageState();
}

class _CameraListPageState extends ConsumerState<CameraListPage> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(cameraProvider.notifier).loadCameras();
    });
  }

  @override
  Widget build(BuildContext context) {
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
        return const Center(child: CircularProgressIndicator());
      case CameraLoadStatus.loading:
        return const Center(child: CircularProgressIndicator());
      case CameraLoadStatus.error:
        return Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(Icons.error_outline, size: 48, color: AppColors.error),
              const SizedBox(height: 16),
              Text(
                provider.errorMessage ?? 'Failed to load cameras',
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: () => ref.read(cameraProvider.notifier).loadCameras(),
                child: const Text('Retry'),
              ),
            ],
          ),
        );
      case CameraLoadStatus.loaded:
        if (provider.cameras.isEmpty) {
          return const Center(child: Text('No cameras found'));
        }
        return RefreshIndicator(
          onRefresh: () => ref.read(cameraProvider.notifier).loadCameras(),
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
