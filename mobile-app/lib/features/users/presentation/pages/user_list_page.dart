import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/user_provider.dart';
import '../widgets/user_card.dart';

class UserListPage extends ConsumerWidget {
  const UserListPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifier = ref.watch(userListProvider);
    final state = ref.read(userListProvider.notifier);

    return Scaffold(
      appBar: AppBar(title: const Text('Users')),
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
                        onPressed: () => state.loadUsers(refresh: true),
                        child: const Text('Retry'),
                      ),
                    ],
                  ),
                )
              : notifier.users.isEmpty
                  ? const Center(child: Text('No users found'))
                  : ListView.builder(
                      itemCount: notifier.users.length,
                      padding: const EdgeInsets.all(8),
                      itemBuilder: (context, index) {
                        final item = notifier.users[index];
                        return UserCard(
                          user: item,
                          onTap: () {
                            // TODO: navigate to user detail
                          },
                          onDelete: () => state.deleteUser(item.id),
                        );
                      },
                    ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          // TODO: navigate to create user page
        },
        child: const Icon(Icons.add),
      ),
    );
  }
}
