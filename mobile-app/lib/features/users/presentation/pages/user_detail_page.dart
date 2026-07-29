import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/user_provider.dart';

class UserDetailPage extends ConsumerWidget {
  final String userId;

  const UserDetailPage({super.key, required this.userId});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifier = ref.watch(userListProvider);
    final state = ref.read(userListProvider.notifier);

    return Scaffold(
      appBar: AppBar(title: const Text('User Details')),
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
                        onPressed: () => state.loadUserById(userId),
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                )
              : notifier.selectedUser == null
                  ? const Center(child: Text('User not found'))
                  : Padding(
                      padding: const EdgeInsets.all(16),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Center(
                            child: CircleAvatar(
                              radius: 48,
                              child: Text(
                                '${notifier.selectedUser!.firstName[0]}${notifier.selectedUser!.lastName[0]}',
                                style: const TextStyle(fontSize: 32, fontWeight: FontWeight.bold),
                              ),
                            ),
                          ),
                          const SizedBox(height: 24),
                          _detailRow('Name', notifier.selectedUser!.fullName),
                          _detailRow('Email', notifier.selectedUser!.email),
                          _detailRow('Role', notifier.selectedUser!.role),
                          _detailRow('Status', notifier.selectedUser!.enabled ? 'Enabled' : 'Disabled'),
                          _detailRow('Created', notifier.selectedUser!.createdAt.toString()),
                          if (notifier.selectedUser!.updatedAt != null)
                            _detailRow('Updated', notifier.selectedUser!.updatedAt.toString()),
                        ],
                      ),
                    ),
    );
  }

  Widget _detailRow(String label, String value) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 100,
            child: Text(label, style: const TextStyle(fontWeight: FontWeight.w600, color: Colors.grey)),
          ),
          Expanded(child: Text(value)),
        ],
      ),
    );
  }
}
