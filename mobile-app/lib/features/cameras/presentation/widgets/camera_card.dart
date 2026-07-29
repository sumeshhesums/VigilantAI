import 'package:flutter/material.dart';

import '../../domain/entities/camera.dart';
import 'camera_status_badge.dart';

class CameraCard extends StatelessWidget {
  final Camera camera;
  final VoidCallback? onTap;

  const CameraCard({super.key, required this.camera, this.onTap});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
      child: ListTile(
        onTap: onTap,
        leading: Icon(
          camera.enabled ? Icons.videocam : Icons.videocam_off,
          color: camera.enabled ? Colors.blue : Colors.grey,
        ),
        title: Text(
          camera.name,
          style: const TextStyle(fontWeight: FontWeight.w600),
        ),
        subtitle: Text(camera.location),
        trailing: CameraStatusBadge(status: camera.status),
      ),
    );
  }
}
