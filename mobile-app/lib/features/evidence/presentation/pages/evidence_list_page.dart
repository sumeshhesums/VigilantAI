import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/evidence_provider.dart';
import '../widgets/evidence_card.dart';

class EvidenceListPage extends ConsumerWidget {
  final String incidentId;

  const EvidenceListPage({super.key, required this.incidentId});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final provider = evidenceListProvider(incidentId);
    final notifier = ref.watch(provider);
    final state = ref.read(provider.notifier);

    return Scaffold(
      appBar: AppBar(title: const Text('Evidence')),
      body: notifier.isLoading
          ? const Center(child: CircularProgressIndicator())
          : notifier.errorMessage != null
              ? Center(
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(notifier.errorMessage!),
                      const SizedBox(height: 8),
                      ElevatedButton(
                        onPressed: () => state.loadEvidence(refresh: true),
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                )
              : notifier.evidence.isEmpty
                  ? const Center(child: Text('No evidence found'))
                  : ListView.builder(
                      itemCount: notifier.evidence.length,
                      padding: const EdgeInsets.all(8),
                      itemBuilder: (context, index) {
                        final item = notifier.evidence[index];
                        return EvidenceCard(evidence: item);
                      },
                    ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          // TODO: navigate to upload evidence page
        },
        child: const Icon(Icons.upload),
      ),
    );
  }
}
