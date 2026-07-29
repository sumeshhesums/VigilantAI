import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../data/datasources/user_remote_datasource.dart';
import '../../data/repositories/user_repository_impl.dart';
import '../../domain/entities/user.dart';
import '../../domain/repositories/user_repository.dart';

final userRepositoryProvider = Provider<UserRepository>((ref) {
  final dataSource = ref.watch(userRemoteDataSourceProvider);
  return UserRepositoryImpl(dataSource);
});

final userRemoteDataSourceProvider = Provider<UserRemoteDataSource>((ref) {
  throw UnimplementedError('ApiClient provider not implemented');
});

final userListProvider = ChangeNotifierProvider<UserListNotifier>((ref) {
  return UserListNotifier(ref.read(userRepositoryProvider));
});

class UserListNotifier extends ChangeNotifier {
  final UserRepository _repository;
  List<User> _users = [];
  User? _selectedUser;
  bool _isLoading = false;
  String? _errorMessage;

  UserListNotifier(this._repository);

  List<User> get users => _users;
  User? get selectedUser => _selectedUser;
  bool get isLoading => _isLoading;
  String? get errorMessage => _errorMessage;

  Future<void> loadUsers({bool refresh = false}) async {
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    final result = await _repository.getUsers();
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (list) {
        _users = list;
      },
    );

    _isLoading = false;
    notifyListeners();
  }

  Future<void> loadUserById(String id) async {
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    final result = await _repository.getUserById(id);
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (user) {
        _selectedUser = user;
      },
    );

    _isLoading = false;
    notifyListeners();
  }

  Future<void> createUser({
    required String email,
    required String password,
    required String firstName,
    required String lastName,
    required List<String> roles,
  }) async {
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    final result = await _repository.createUser(
      email: email,
      password: password,
      firstName: firstName,
      lastName: lastName,
      roles: roles,
    );
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (user) {
        _users.add(user);
      },
    );

    _isLoading = false;
    notifyListeners();
  }

  Future<void> updateUser(String id, {String? email, String? firstName, String? lastName}) async {
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    final result = await _repository.updateUser(
      id,
      email: email,
      firstName: firstName,
      lastName: lastName,
    );
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (user) {
        final index = _users.indexWhere((u) => u.id == id);
        if (index != -1) _users[index] = user;
        _selectedUser = user;
      },
    );

    _isLoading = false;
    notifyListeners();
  }

  Future<void> deleteUser(String id) async {
    final result = await _repository.deleteUser(id);
    result.fold(
      (failure) {
        _errorMessage = failure.message;
        notifyListeners();
      },
      (_) {
        _users.removeWhere((u) => u.id == id);
        notifyListeners();
      },
    );
  }

  void clearError() {
    _errorMessage = null;
    notifyListeners();
  }
}
