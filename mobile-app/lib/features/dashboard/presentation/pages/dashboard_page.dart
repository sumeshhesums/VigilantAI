import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../core/theme/app_colors.dart';
import '../providers/dashboard_provider.dart';
import '../widgets/kpi_card.dart';
import '../widgets/live_stats_card.dart';

class DashboardPage extends ConsumerStatefulWidget {
  const DashboardPage({super.key});

  @override
  ConsumerState<DashboardPage> createState() => _DashboardPageState();
}

class _DashboardPageState extends ConsumerState<DashboardPage> {
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      ref.read(dashboardProvider.notifier).loadDashboard();
    });
  }

  @override
  Widget build(BuildContext context) {
    final notifier = ref.watch(dashboardProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Dashboard'),
        actions: [
          IconButton(
            icon: const Icon(Icons.notifications_outlined),
            onPressed: () {},
          ),
          IconButton(
            icon: const Icon(Icons.person_outline),
            onPressed: () {},
          ),
        ],
      ),
      body: notifier.isLoading
          ? const Center(child: CircularProgressIndicator())
          : notifier.errorMessage != null
              ? Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.error_outline, size: 48, color: AppColors.error),
                      const SizedBox(height: 16),
                      Text(
                        notifier.errorMessage!,
                        textAlign: TextAlign.center,
                      ),
                      const SizedBox(height: 16),
                      ElevatedButton(
                        onPressed: () => ref.read(dashboardProvider.notifier).loadDashboard(),
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                )
              : RefreshIndicator(
                  onRefresh: () => ref.read(dashboardProvider.notifier).loadDashboard(),
                  child: SingleChildScrollView(
                    physics: const AlwaysScrollableScrollPhysics(),
                    padding: const EdgeInsets.all(16),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          'Overview',
                          style: TextStyle(
                            fontSize: 20,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        const SizedBox(height: 16),
                        _buildKpiGrid(notifier),
                        const SizedBox(height: 24),
                        const Text(
                          'Live Status',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        const SizedBox(height: 12),
                        _buildLiveStatsList(notifier),
                        const SizedBox(height: 24),
                        const Text(
                          'Incidents Summary',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                        const SizedBox(height: 12),
                        _buildIncidentsSummary(notifier),
                      ],
                    ),
                  ),
                ),
    );
  }

  Widget _buildKpiGrid(DashboardNotifier notifier) {
    final kpis = notifier.kpis;
    return GridView.count(
      crossAxisCount: 2,
      shrinkWrap: true,
      physics: const NeverScrollableScrollPhysics(),
      mainAxisSpacing: 12,
      crossAxisSpacing: 12,
      childAspectRatio: 1.5,
      children: [
        KpiCard(
          title: 'Total Cameras',
          value: kpis != null ? '${kpis.activeCameras}' : '--',
          icon: Icons.videocam,
          color: AppColors.primary,
        ),
        KpiCard(
          title: 'Online',
          value: kpis != null ? '${kpis.onlineCameras}' : '--',
          icon: Icons.check_circle,
          color: AppColors.online,
        ),
        KpiCard(
          title: 'Offline',
          value: kpis != null ? '${kpis.offlineCameras}' : '--',
          icon: Icons.error,
          color: AppColors.offline,
        ),
        KpiCard(
          title: 'Detections (24h)',
          value: kpis != null ? '${kpis.totalDetections24h}' : '--',
          icon: Icons.visibility,
          color: AppColors.info,
        ),
        KpiCard(
          title: 'Critical Alerts',
          value: kpis != null ? '${kpis.criticalAlerts}' : '--',
          icon: Icons.warning,
          color: AppColors.critical,
        ),
        KpiCard(
          title: 'Open Incidents',
          value: kpis != null ? '${kpis.openIncidents}' : '--',
          icon: Icons.shield,
          color: AppColors.high,
        ),
        KpiCard(
          title: 'Avg Response',
          value: kpis != null ? '${kpis.avgResponseTimeSeconds.toStringAsFixed(1)}s' : '--',
          icon: Icons.timer,
          color: AppColors.medium,
        ),
        KpiCard(
          title: 'SLA Compliance',
          value: kpis != null ? '${kpis.slaCompliancePercent.toStringAsFixed(1)}%' : '--',
          icon: Icons.verified,
          color: kpis != null && kpis.slaCompliancePercent >= 95 ? AppColors.success : AppColors.warning,
        ),
      ],
    );
  }

  Widget _buildLiveStatsList(DashboardNotifier notifier) {
    final stats = notifier.liveStats;
    if (stats == null) {
      return const Card(
        child: Padding(
          padding: EdgeInsets.all(16),
          child: Text('Live stats unavailable'),
        ),
      );
    }
    return Column(
      children: [
        LiveStatsCard(
          label: 'Active Alerts',
          value: '${stats.activeAlerts}',
          icon: Icons.notifications_active,
          color: AppColors.critical,
        ),
        const SizedBox(height: 8),
        LiveStatsCard(
          label: 'Cameras Online',
          value: '${stats.camerasOnline}',
          icon: Icons.videocam,
          color: AppColors.online,
        ),
        const SizedBox(height: 8),
        LiveStatsCard(
          label: 'Cameras Offline',
          value: '${stats.camerasOffline}',
          icon: Icons.videocam_off,
          color: AppColors.offline,
        ),
        const SizedBox(height: 8),
        LiveStatsCard(
          label: 'Detections Today',
          value: '${stats.detectionsToday}',
          icon: Icons.visibility,
          color: AppColors.info,
        ),
        const SizedBox(height: 8),
        LiveStatsCard(
          label: 'Uptime',
          value: '${stats.uptimePercentage.toStringAsFixed(1)}%',
          icon: Icons.trending_up,
          color: stats.uptimePercentage >= 99 ? AppColors.success : AppColors.warning,
        ),
        const SizedBox(height: 8),
        LiveStatsCard(
          label: 'Avg FPS',
          value: '${stats.avgFps.toStringAsFixed(1)}',
          icon: Icons.speed,
          color: AppColors.primary,
        ),
      ],
    );
  }

  Widget _buildIncidentsSummary(DashboardNotifier notifier) {
    final summary = notifier.incidentsSummary;
    if (summary == null) {
      return const Card(
        child: Padding(
          padding: EdgeInsets.all(16),
          child: Text('Incidents summary unavailable'),
        ),
      );
    }
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Total: ${summary.total}',
              style: const TextStyle(fontWeight: FontWeight.w600),
            ),
            const SizedBox(height: 12),
            const Text('By Status', style: TextStyle(fontWeight: FontWeight.w500)),
            const SizedBox(height: 4),
            ...summary.byStatus.map((item) => Padding(
              padding: const EdgeInsets.symmetric(vertical: 4),
              child: Row(
                children: [
                  SizedBox(
                    width: 100,
                    child: Text(item.status, style: Theme.of(context).textTheme.bodySmall),
                  ),
                  Expanded(
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(4),
                      child: LinearProgressIndicator(
                        value: item.percentage / 100,
                        minHeight: 8,
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  Text(
                    '${item.count} (${item.percentage.toStringAsFixed(0)}%)',
                    style: Theme.of(context).textTheme.bodySmall?.copyWith(fontWeight: FontWeight.w600),
                  ),
                ],
              ),
            )),
            const SizedBox(height: 12),
            const Text('By Severity', style: TextStyle(fontWeight: FontWeight.w500)),
            const SizedBox(height: 4),
            ...summary.bySeverity.map((item) => Padding(
              padding: const EdgeInsets.symmetric(vertical: 4),
              child: Row(
                children: [
                  SizedBox(
                    width: 100,
                    child: Text(item.status, style: Theme.of(context).textTheme.bodySmall),
                  ),
                  Expanded(
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(4),
                      child: LinearProgressIndicator(
                        value: item.percentage / 100,
                        minHeight: 8,
                      ),
                    ),
                  ),
                  const SizedBox(width: 8),
                  Text(
                    '${item.count} (${item.percentage.toStringAsFixed(0)}%)',
                    style: Theme.of(context).textTheme.bodySmall?.copyWith(fontWeight: FontWeight.w600),
                  ),
                ],
              ),
            )),
          ],
        ),
      ),
    );
  }
}
