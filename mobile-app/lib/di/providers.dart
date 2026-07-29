import 'package:connectivity_plus/connectivity_plus.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

import '../core/network/api_client.dart';
import '../core/network/dio_client.dart';
import '../core/network/network_info.dart';
import '../features/auth/data/datasources/auth_remote_datasource.dart';
import '../features/auth/data/repositories/auth_repository_impl.dart';
import '../features/auth/domain/repositories/auth_repository.dart';
import '../features/cameras/data/datasources/camera_remote_datasource.dart';
import '../features/cameras/data/repositories/camera_repository_impl.dart';
import '../features/cameras/domain/repositories/camera_repository.dart';
import '../features/dashboard/data/datasources/dashboard_remote_datasource.dart';
import '../features/dashboard/data/repositories/dashboard_repository_impl.dart';
import '../features/dashboard/domain/repositories/dashboard_repository.dart';
import '../features/evidence/data/datasources/evidence_remote_datasource.dart';
import '../features/evidence/data/repositories/evidence_repository_impl.dart';
import '../features/evidence/domain/repositories/evidence_repository.dart';
import '../features/incidents/data/datasources/incident_remote_datasource.dart';
import '../features/incidents/data/repositories/incident_repository_impl.dart';
import '../features/incidents/domain/repositories/incident_repository.dart';
import '../features/notifications/data/datasources/notification_remote_datasource.dart';
import '../features/notifications/data/repositories/notification_repository_impl.dart';
import '../features/notifications/domain/repositories/notification_repository.dart';
import '../features/roles/data/datasources/role_remote_datasource.dart';
import '../features/roles/data/repositories/role_repository_impl.dart';
import '../features/roles/domain/repositories/role_repository.dart';
import '../features/users/data/datasources/user_remote_datasource.dart';
import '../features/users/data/repositories/user_repository_impl.dart';
import '../features/users/domain/repositories/user_repository.dart';

// MARK: - Core
final apiClientProvider = Provider<ApiClient>((ref) => DioClient());

final secureStorageProvider = Provider<FlutterSecureStorage>(
    (ref) => const FlutterSecureStorage());

final networkInfoProvider = Provider<NetworkInfo>(
    (ref) => NetworkInfo(Connectivity()));

// MARK: - Auth
final authRemoteDataSourceProvider =
    Provider<AuthRemoteDataSource>((ref) =>
        AuthRemoteDataSourceImpl(ref.watch(apiClientProvider)));

final authRepositoryProvider = Provider<AuthRepository>((ref) =>
    AuthRepositoryImpl(
      ref.watch(authRemoteDataSourceProvider),
      ref.watch(apiClientProvider),
      ref.watch(secureStorageProvider),
    ));

final authStatusProvider = FutureProvider<bool>((ref) async {
  final repo = ref.watch(authRepositoryProvider);
  final result = await repo.isAuthenticated();
  return result.getOrElse(() => false);
});

// MARK: - Dashboard
final dashboardRemoteDataSourceProvider =
    Provider<DashboardRemoteDataSource>((ref) =>
        DashboardRemoteDataSourceImpl(ref.watch(apiClientProvider)));

final dashboardRepositoryProvider = Provider<DashboardRepository>((ref) =>
    DashboardRepositoryImpl(ref.watch(dashboardRemoteDataSourceProvider)));

// MARK: - Cameras
final cameraRemoteDataSourceProvider =
    Provider<CameraRemoteDataSource>((ref) =>
        CameraRemoteDataSourceImpl(ref.watch(apiClientProvider)));

final cameraRepositoryProvider = Provider<CameraRepository>((ref) =>
    CameraRepositoryImpl(ref.watch(cameraRemoteDataSourceProvider)));

// MARK: - Incidents
final incidentRemoteDataSourceProvider =
    Provider<IncidentRemoteDataSource>((ref) =>
        IncidentRemoteDataSourceImpl(ref.watch(apiClientProvider)));

final incidentRepositoryProvider = Provider<IncidentRepository>((ref) =>
    IncidentRepositoryImpl(ref.watch(incidentRemoteDataSourceProvider)));

// MARK: - Evidence
final evidenceRemoteDataSourceProvider =
    Provider<EvidenceRemoteDataSource>((ref) =>
        EvidenceRemoteDataSourceImpl(ref.watch(apiClientProvider)));

final evidenceRepositoryProvider = Provider<EvidenceRepository>((ref) =>
    EvidenceRepositoryImpl(ref.watch(evidenceRemoteDataSourceProvider)));

// MARK: - Notifications
final notificationRemoteDataSourceProvider =
    Provider<NotificationRemoteDataSource>((ref) =>
        NotificationRemoteDataSourceImpl(ref.watch(apiClientProvider)));

final notificationRepositoryProvider = Provider<NotificationRepository>(
    (ref) =>
        NotificationRepositoryImpl(
            ref.watch(notificationRemoteDataSourceProvider)));

// MARK: - Users
final userRemoteDataSourceProvider =
    Provider<UserRemoteDataSource>((ref) =>
        UserRemoteDataSourceImpl(ref.watch(apiClientProvider)));

final userRepositoryProvider = Provider<UserRepository>((ref) =>
    UserRepositoryImpl(ref.watch(userRemoteDataSourceProvider)));

// MARK: - Roles
final roleRemoteDataSourceProvider =
    Provider<RoleRemoteDataSource>((ref) =>
        RoleRemoteDataSourceImpl(ref.watch(apiClientProvider)));

final roleRepositoryProvider = Provider<RoleRepository>((ref) =>
    RoleRepositoryImpl(ref.watch(roleRemoteDataSourceProvider)));
