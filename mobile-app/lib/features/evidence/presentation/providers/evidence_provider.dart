import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../data/datasources/evidence_remote_datasource.dart';
import '../../data/repositories/evidence_repository_impl.dart';
import '../../domain/entities/evidence.dart';
import '../../domain/repositories/evidence_repository.dart';

final evidenceRepositoryProvider = Provider<EvidenceRepository>((ref) {
  final dataSource = ref.watch(evidenceRemoteDataSourceProvider);
  return EvidenceRepositoryImpl(dataSource);
});

final evidenceRemoteDataSourceProvider = Provider<EvidenceRemoteDataSource>((ref) {
  throw UnimplementedError('ApiClient provider not implemented');
});

final evidenceListProvider = ChangeNotifierProvider<EvidenceListNotifier>((ref) {
  return EvidenceListNotifier(ref.read(evidenceRepositoryProvider));
});

class EvidenceListNotifier extends ChangeNotifier {
  final EvidenceRepository _repository;
  List<Evidence> _evidence = [];
  bool _isLoading = false;
  String? _errorMessage;
  int _currentPage = 1;
  int _total = 0;

  EvidenceListNotifier(this._repository);

  List<Evidence> get evidence => _evidence;
  bool get isLoading => _isLoading;
  String? get errorMessage => _errorMessage;
  int get currentPage => _currentPage;
  int get total => _total;

  Future<void> loadEvidence({bool refresh = false}) async {
    if (refresh) _currentPage = 1;
    _isLoading = true;
    _errorMessage = null;
    notifyListeners();

    final result = await _repository.getEvidence(page: _currentPage);
    result.fold(
      (failure) {
        _errorMessage = failure.message;
      },
      (list) {
        _evidence = refresh ? list : [..._evidence, ...list];
        _total = list.length;
      },
    );

    _isLoading = false;
    notifyListeners();
  }

  Future<void> deleteEvidence(String id) async {
    final result = await _repository.deleteEvidence(id);
    result.fold(
      (failure) {
        _errorMessage = failure.message;
        notifyListeners();
      },
      (_) {
        _evidence.removeWhere((e) => e.id == id);
        notifyListeners();
      },
    );
  }

  void clearError() {
    _errorMessage = null;
    notifyListeners();
  }
}
