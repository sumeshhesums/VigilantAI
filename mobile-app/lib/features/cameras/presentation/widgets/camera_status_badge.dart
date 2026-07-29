import 'package:flutter/material.dart';

class CameraStatusBadge extends StatelessWidget {
  final String status;

  const CameraStatusBadge({super.key, required this.status});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: _backgroundColor,
        borderRadius: BorderRadius.circular(12),
      ),
      child: Text(
        status.toUpperCase(),
        style: TextStyle(
          color: _textColor,
          fontSize: 11,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }

  Color get _backgroundColor {
    switch (status) {
      case 'online':
        return Colors.green.shade100;
      case 'offline':
        return Colors.grey.shade200;
      case 'error':
        return Colors.red.shade100;
      case 'maintenance':
        return Colors.orange.shade100;
      default:
        return Colors.grey.shade200;
    }
  }

  Color get _textColor {
    switch (status) {
      case 'online':
        return Colors.green.shade800;
      case 'offline':
        return Colors.grey.shade700;
      case 'error':
        return Colors.red.shade800;
      case 'maintenance':
        return Colors.orange.shade800;
      default:
        return Colors.grey.shade700;
    }
  }
}
