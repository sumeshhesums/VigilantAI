import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../di/providers.dart';
import '../../domain/repositories/auth_repository.dart';

enum AuthState { initial, loading, authenticated, unauthenticated, error }

final authStateProvider =
    StateNotifierProvider<AuthNotifier, AuthState>((ref) {
  final repo = ref.watch(authRepositoryProvider);
  return AuthNotifier(repo);
});

class AuthNotifier extends StateNotifier<AuthState> {
  final AuthRepository _repository;

  AuthNotifier(this._repository) : super(AuthState.initial);

  Future<void> login(String email, String password) async {
    state = AuthState.loading;
    final result = await _repository.login(email, password);
    result.fold(
      (failure) => state = AuthState.error,
      (_) => state = AuthState.authenticated,
    );
  }

  Future<void> logout() async {
    await _repository.logout();
    state = AuthState.unauthenticated;
  }

  Future<void> checkAuth() async {
    final result = await _repository.isAuthenticated();
    result.fold(
      (_) => state = AuthState.unauthenticated,
      (isAuth) => state = isAuth ? AuthState.authenticated : AuthState.unauthenticated,
    );
  }
}
