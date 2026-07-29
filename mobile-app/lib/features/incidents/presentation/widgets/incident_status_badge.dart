import 'package:flutter/material.dart';

import '../../../../core/theme/app_colors.dart';

class IncidentStatusBadge extends StatelessWidget {
  final String status;

  const IncidentStatusBadge({super.key, required this.status});

  @override
  Widget build(BuildContext context) {
    final (Color color, IconData icon) = _getStatusStyle(status);

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
      decoration: BoxDecoration(
        color: color.withOpacity(0.12),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color.withOpacity(0.3)),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(icon, size: 14, color: color),
          const SizedBox(width: 4),
          Text(
            status.toUpperCase(),
            style: TextStyle(
              fontSize: 11,
              fontWeight: FontWeight.w600,
              color: color,
              letterSpacing: 0.5,
            ),
          ),
        ],
      ),
    );
  }

  (Color, IconData) _getStatusStyle(String status) {
    switch (status.toLowerCase()) {
      case 'open':
        return (AppColors.info, Icons.radio_button_unchecked);
      case 'acknowledged':
        return (AppColors.warning, Icons.visibility);
      case 'investigating':
        return (AppColors.high, Icons.search);
      case 'resolved':
        return (AppColors.success, Icons.check_circle_outline);
      case 'closed':
        return (AppColors.textSecondary, Icons.cancel_outlined);
      default:
        return (AppColors.textSecondary, Icons.help_outline);
    }
  }
}
