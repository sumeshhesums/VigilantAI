import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../di/providers.dart';
import '../../domain/entities/evidence.dart';
import '../../domain/repositories/evidence_repository.dart';

final evidenceListProvider = ChangeNotifierProvider.family<EvidenceListNotifier, String>((ref, incidentId) {
  return EvidenceListNotifier(ref.read(evidenceRepositoryProvider), incidentId);
});

class EvidenceListNotifier extends ChangeNotifier {
  final EvidenceRepository _repository;
  final String _incidentId;
  List<Evidence> _evidence = [];
  bool _isLoading = false;
  String? _errorMessage;
  int _currentPage = 1;
  int _total = 0;

  EvidenceListNotifier(this._repository, this._incidentId);

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

    final result = await _repository.getEvidenceByIncident(_incidentId, page: _currentPage);
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

  void clearError() {
    _errorMessage = null;
    notifyListeners();
  }
}
