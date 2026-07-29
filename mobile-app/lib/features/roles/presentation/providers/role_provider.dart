import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../data/datasources/role_remote_datasource.dart';
import '../../data/repositories/role_repository_impl.dart';
import '../../domain/entities/role.dart';
import '../../domain/repositories/role_repository.dart';

final roleRepositoryProvider = Provider<RoleRepository>((ref) {
  final dataSource = ref.watch(roleRemoteDataSourceProvider);
  return RoleRepositoryImpl(dataSource);
});

final roleRemoteDataSourceProvider = Provider<RoleRemoteDataSource>((ref) {
  throw UnimplementedError('ApiClient provider not implemented');
});

final roleListProvider = ChangeNotifierProvider<RoleListNotifier>((ref) {
  return RoleListNotifier(ref.read(roleRepositoryProvider));
});

class RoleListNotifier extends ChangeNotifier {
  final RoleRepository _repository;
  List<Role> _roles = [];
  bool _isLoading = false;
  String? _errorMessage;

  RoleListNotifier(this._repository);

  List<Role> get roles => _roles;
  bool get isLoading => _isLoading;
  String? get errorMessage => _errorMessage;

  Future<void> loadRoles({bool refresh = false}) async {
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    final result = await _repository.getRoles();
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (list) {
        _roles = list;
      },
    );

    _isLoading = false;
    notifyListeners();
  }

  void clearError() {
    _errorMessage = null;
    notifyListeners();
  }
}
