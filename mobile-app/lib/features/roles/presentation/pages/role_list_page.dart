import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/role_provider.dart';
import '../widgets/role_card.dart';

class RoleListPage extends ConsumerWidget {
  const RoleListPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifier = ref.watch(roleListProvider);
    final state = ref.read(roleListProvider.notifier);

    return Scaffold(
      appBar: AppBar(title: const Text('Roles')),
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
                        onPressed: () => state.loadRoles(refresh: true),
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                )
              : notifier.roles.isEmpty
                  ? const Center(child: Text('No roles found'))
                  : RefreshIndicator(
                      onRefresh: () => state.loadRoles(refresh: true),
                      child: ListView.builder(
                        itemCount: notifier.roles.length,
                        padding: const EdgeInsets.symmetric(vertical: 8),
                        itemBuilder: (context, index) {
                          final item = notifier.roles[index];
                          return RoleCard(role: item);
                        },
                      ),
                    ),
    );
  }
}
