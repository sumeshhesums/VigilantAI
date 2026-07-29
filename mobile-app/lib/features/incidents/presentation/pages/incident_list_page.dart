import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../core/theme/app_colors.dart';
import '../providers/incident_provider.dart';
import '../widgets/incident_card.dart';
import 'incident_detail_page.dart';

class IncidentListPage extends ConsumerWidget {
  const IncidentListPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifier = ref.watch(incidentProvider);
    final provider = ref.read(incidentProvider.notifier);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Incidents'),
        actions: [
          IconButton(
            icon: const Icon(Icons.filter_list),
            onPressed: () {},
          ),
        ],
      ),
      body: _buildBody(context, ref, notifier, provider),
    );
  }

  Widget _buildBody(
    BuildContext context,
    WidgetRef ref,
    IncidentNotifier notifier,
    IncidentNotifier provider,
  ) {
    switch (notifier.state) {
      case IncidentLoadingState.initial:
        WidgetsBinding.instance.addPostFrameCallback((_) {
          provider.loadIncidents(refresh: true);
        });
        return const Center(child: CircularProgressIndicator());

      case IncidentLoadingState.loading:
        if (notifier.incidents.isEmpty) {
          return const Center(child: CircularProgressIndicator());
        }
        return _buildList(context, ref, notifier);

      case IncidentLoadingState.loaded:
        if (notifier.incidents.isEmpty) {
          return Center(
            child: Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Icon(Icons.shield_outlined, size: 64, color: AppColors.textSecondary.withOpacity(0.5)),
                const SizedBox(height: 16),
                Text(
                  'No incidents found',
                  style: TextStyle(fontSize: 16, color: AppColors.textSecondary),
                ),
              ],
            ),
          );
        }
        return _buildList(context, ref, notifier);

      case IncidentLoadingState.error:
        return Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Icon(Icons.error_outline, size: 48, color: AppColors.error),
              const SizedBox(height: 16),
              Text(
                notifier.errorMessage ?? 'Something went wrong',
                textAlign: TextAlign.center,
                style: const TextStyle(color: AppColors.textSecondary),
              ),
              const SizedBox(height: 16),
              ElevatedButton(
                onPressed: () => provider.loadIncidents(refresh: true),
                child: const Text('Retry'),
              ),
            ],
          ),
        );
    }
  }

  Widget _buildList(BuildContext context, WidgetRef ref, IncidentNotifier notifier) {
    return NotificationListener<ScrollNotification>(
      onNotification: (notification) {
        if (notification is ScrollEndNotification &&
            notification.metrics.pixels >= notification.metrics.maxScrollExtent - 200) {
          if (notifier.hasMore && !notifier.isLoading) {
            ref.read(incidentProvider.notifier).loadMore();
          }
        }
        return false;
      },
      child: RefreshIndicator(
        onRefresh: () async {
          ref.read(incidentProvider.notifier).loadIncidents(refresh: true);
        },
        child: ListView.builder(
          padding: const EdgeInsets.symmetric(vertical: 8),
          itemCount: notifier.incidents.length + (notifier.hasMore ? 1 : 0),
          itemBuilder: (context, index) {
            if (index >= notifier.incidents.length) {
              return const Padding(
                padding: EdgeInsets.all(16),
                child: Center(child: CircularProgressIndicator()),
              );
            }

            final incident = notifier.incidents[index];
            return IncidentCard(
              incident: incident,
              onTap: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (_) => IncidentDetailPage(incidentId: incident.id),
                  ),
                );
              },
            );
          },
        ),
      ),
    );
  }
}
