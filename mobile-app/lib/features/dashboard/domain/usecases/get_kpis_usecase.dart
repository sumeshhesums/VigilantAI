import 'package:dartz/dartz.dart';

import '../../../../core/errors/failures.dart';
import '../entities/kpi.dart';
import '../repositories/dashboard_repository.dart';

class GetKpisUseCase {
  final DashboardRepository _repository;

  GetKpisUseCase(this._repository);

  Future<Either<Failure, Kpi>> execute() => _repository.getKpis();
}
