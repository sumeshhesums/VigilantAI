import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../../di/providers.dart';
import '../../domain/entities/incident.dart';
import '../../domain/repositories/incident_repository.dart';

final incidentProvider = ChangeNotifierProvider<IncidentNotifier>((ref) {
  return IncidentNotifier(repository: ref.watch(incidentRepositoryProvider));
});

enum IncidentLoadingState { initial, loading, loaded, error }

class IncidentNotifier extends ChangeNotifier {
  final IncidentRepository? _repository;

  IncidentLoadingState _state = IncidentLoadingState.initial;
  IncidentLoadingState _detailState = IncidentLoadingState.initial;
  List<Incident> _incidents = [];
  Incident? _selectedIncident;
  PaginatedIncidents? _paginatedData;
  String? _errorMessage;
  int _currentPage = 1;
  static const int _pageSize = 20;

  IncidentNotifier({IncidentRepository? repository}) : _repository = repository;

  IncidentLoadingState get state => _state;
  IncidentLoadingState get detailState => _detailState;
  List<Incident> get incidents => _incidents;
  Incident? get selectedIncident => _selectedIncident;
  PaginatedIncidents? get paginatedData => _paginatedData;
  String? get errorMessage => _errorMessage;
  int get currentPage => _currentPage;
  bool get isLoading => _state == IncidentLoadingState.loading;
  bool get hasMore => _paginatedData == null || _currentPage * _pageSize < _paginatedData!.total;

  void loadIncidents({bool refresh = false}) {
    if (refresh) {
      _currentPage = 1;
      _incidents.clear();
    }

    _state = IncidentLoadingState.loading;
    notifyListeners();

    if (_repository == null) {
      Future.delayed(const Duration(milliseconds: 500), () {
        _state = IncidentLoadingState.loaded;
        notifyListeners();
      });
      return;
    }

    _repository.getIncidents(page: _currentPage, perPage: _pageSize).then((result) {
      result.fold(
        (failure) {
          _state = IncidentLoadingState.error;
          _errorMessage = failure.message;
          notifyListeners();
        },
        (data) {
          _paginatedData = data;
          if (refresh) {
            _incidents = data.incidents;
          } else {
            _incidents.addAll(data.incidents);
          }
          _state = IncidentLoadingState.loaded;
          notifyListeners();
        },
      );
    });
  }

  void loadMore() {
    if (!hasMore || isLoading) return;
    _currentPage++;
    loadIncidents();
  }

  void loadIncidentById(String id) {
    _detailState = IncidentLoadingState.loading;
    notifyListeners();

    if (_repository == null) {
      Future.delayed(const Duration(milliseconds: 300), () {
        _detailState = IncidentLoadingState.error;
        _errorMessage = 'Repository not available';
        notifyListeners();
      });
      return;
    }

    _repository.getIncidentById(id).then((result) {
      result.fold(
        (failure) {
          _detailState = IncidentLoadingState.error;
          _errorMessage = failure.message;
          notifyListeners();
        },
        (incident) {
          _selectedIncident = incident;
          _detailState = IncidentLoadingState.loaded;
          notifyListeners();
        },
      );
    });
  }

  void clearError() {
    _errorMessage = null;
    notifyListeners();
  }
}
