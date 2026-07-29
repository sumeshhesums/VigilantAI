import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

final dashboardProvider =
    ChangeNotifierProvider<DashboardNotifier>((ref) {
  return DashboardNotifier();
});

class DashboardNotifier extends ChangeNotifier {
  bool _isLoading = false;

  bool get isLoading => _isLoading;

  void loadDashboard() {
    _isLoading = true;
    notifyListeners();

    // TODO: implement via use cases
    Future.delayed(const Duration(milliseconds: 500), () {
      _isLoading = false;
      notifyListeners();
    });
  }
}
