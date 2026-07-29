import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../core/theme/app_colors.dart';
import '../../domain/entities/incident.dart';
import '../providers/incident_provider.dart';
import '../widgets/incident_status_badge.dart';

class IncidentDetailPage extends ConsumerStatefulWidget {
  final String incidentId;

  const IncidentDetailPage({super.key, required this.incidentId});

  @override
  ConsumerState<IncidentDetailPage> createState() => _IncidentDetailPageState();
}

class _IncidentDetailPageState extends ConsumerState<IncidentDetailPage> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(incidentProvider.notifier).loadIncidentById(widget.incidentId);
    });
  }

  @override
  Widget build(BuildContext context) {
    final notifier = ref.watch(incidentProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Incident Details'),
        actions: [
          if (notifier.selectedIncident != null)
            PopupMenuButton<String>(
              onSelected: (value) {},
              itemBuilder: (context) => [
                const PopupMenuItem(value: 'edit', child: Text('Edit')),
                const PopupMenuItem(value: 'delete', child: Text('Delete')),
              ],
            ),
        ],
      ),
      body: _buildBody(notifier),
    );
  }

  Widget _buildBody(IncidentNotifier notifier) {
    switch (notifier.detailState) {
      case IncidentLoadingState.initial:
      case IncidentLoadingState.loading:
        return const Center(child: CircularProgressIndicator());

      case IncidentLoadingState.error:
        return Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(Icons.error_outline, size: 48, color: AppColors.error),
              const SizedBox(height: 16),
              Text(
                notifier.errorMessage ?? 'Failed to load incident',
                textAlign: TextAlign.center,
                style: const TextStyle(color: AppColors.textSecondary),
              ),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: () {
                  ref.read(incidentProvider.notifier).loadIncidentById(widget.incidentId);
                },
                child: const Text('Retry'),
              ),
            ],
          ),
        );

      case IncidentLoadingState.loaded:
        final incident = notifier.selectedIncident!;
        return SingleChildScrollView(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              _buildHeader(incident),
              const SizedBox(height: 20),
              _buildSection('Description', incident.description),
              const SizedBox(height: 16),
              _buildInfoSection(incident),
            ],
          ),
        );
    }
  }

  Widget _buildHeader(Incident incident) {
    final (Color severityColor, IconData severityIcon) = _getSeverityStyle(incident.severity);

    return Row(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Container(
          padding: const EdgeInsets.all(12),
          decoration: BoxDecoration(
            color: severityColor.withOpacity(0.1),
            borderRadius: BorderRadius.circular(12),
          ),
          child: Icon(severityIcon, color: severityColor, size: 28),
        ),
        const SizedBox(width: 14),
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                incident.title,
                style: const TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 6),
              IncidentStatusBadge(status: incident.status),
              const SizedBox(height: 4),
              Text(
                'Severity: ${incident.severity.toUpperCase()}',
                style: TextStyle(
                  fontSize: 13,
                  color: severityColor,
                  fontWeight: FontWeight.w500,
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildSection(String title, String content) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          title,
          style: const TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
        ),
        const SizedBox(height: 8),
        Container(
          width: double.infinity,
          padding: const EdgeInsets.all(14),
          decoration: BoxDecoration(
            color: AppColors.surfaceDark.withOpacity(0.03),
            borderRadius: BorderRadius.circular(8),
          ),
          child: Text(
            content,
            style: TextStyle(fontSize: 14, color: AppColors.textPrimary.withOpacity(0.85)),
          ),
        ),
      ],
    );
  }

  Widget _buildInfoSection(Incident incident) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Information', style: TextStyle(fontSize: 16, fontWeight: FontWeight.w600)),
        const SizedBox(height: 8),
        _buildInfoRow('Camera', incident.cameraInfo.cameraName),
        _buildInfoRow('Camera ID', incident.cameraInfo.cameraId),
        _buildInfoRow('Detected', _formatDateTime(incident.detectedAt)),
        if (incident.acknowledgedAt != null)
          _buildInfoRow('Acknowledged', _formatDateTime(incident.acknowledgedAt!)),
        if (incident.resolvedAt != null)
          _buildInfoRow('Resolved', _formatDateTime(incident.resolvedAt!)),
        _buildInfoRow('Created', _formatDateTime(incident.createdAt)),
        _buildInfoRow('Updated', _formatDateTime(incident.updatedAt)),
      ],
    );
  }

  Widget _buildInfoRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 6),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 120,
            child: Text(
              label,
              style: TextStyle(fontSize: 13, color: AppColors.textSecondary),
            ),
          ),
          Expanded(
            child: Text(
              value,
              style: const TextStyle(fontSize: 13),
            ),
          ),
        ],
      ),
    );
  }

  String _formatDateTime(DateTime date) {
    return '${date.year}-${date.month.toString().padLeft(2, '0')}-${date.day.toString().padLeft(2, '0')} '
        '${date.hour.toString().padLeft(2, '0')}:${date.minute.toString().padLeft(2, '0')}';
  }

  (Color, IconData) _getSeverityStyle(String severity) {
    switch (severity.toLowerCase()) {
      case 'critical':
        return (AppColors.critical, Icons.error);
      case 'high':
        return (AppColors.high, Icons.warning);
      case 'medium':
        return (AppColors.medium, Icons.info_outline);
      case 'low':
        return (AppColors.low, Icons.check_circle_outline);
      default:
        return (AppColors.textSecondary, Icons.help_outline);
    }
  }
}
