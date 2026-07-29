import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../features/auth/presentation/providers/auth_provider.dart';
import '../../features/auth/presentation/pages/login_page.dart';
import '../../features/splash/presentation/pages/splash_page.dart';
import '../../features/home/presentation/pages/home_shell.dart';
import '../../features/dashboard/presentation/pages/dashboard_page.dart';
import '../../features/cameras/presentation/pages/camera_list_page.dart';
import '../../features/cameras/presentation/pages/camera_detail_page.dart';
import '../../features/incidents/presentation/pages/incident_list_page.dart';
import '../../features/incidents/presentation/pages/incident_detail_page.dart';
import '../../features/notifications/presentation/pages/notification_list_page.dart';
import '../../features/users/presentation/pages/user_list_page.dart';
import '../../features/settings/presentation/pages/settings_page.dart';

final _rootNavigatorKey = GlobalKey<NavigatorState>();
final _shellNavigatorKey = GlobalKey<NavigatorState>();

final appRouterProvider = Provider<GoRouter>((ref) {
  final authState = ref.watch(authStateProvider);

  return GoRouter(
    navigatorKey: _rootNavigatorKey,
    initialLocation: '/splash',
    debugLogDiagnostics: false,
    redirect: (context, state) {
      final isSplash = state.matchedLocation == '/splash';
      final isLogin = state.matchedLocation == '/login';

      if (isSplash) return null;

      final loggedIn = authState == AuthState.authenticated;

      if (!loggedIn && !isLogin) return '/login';
      if (loggedIn && isLogin) return '/';

      return null;
    },
    routes: [
      GoRoute(
        path: '/splash',
        builder: (context, state) => const SplashPage(),
      ),
      GoRoute(
        path: '/login',
        builder: (context, state) => const LoginPage(),
      ),
      ShellRoute(
        navigatorKey: _shellNavigatorKey,
        builder: (context, state, child) => HomeShell(child: child),
        routes: [
          GoRoute(
            path: '/',
            builder: (context, state) => const DashboardPage(),
          ),
          GoRoute(
            path: '/cameras',
            builder: (context, state) => const CameraListPage(),
            routes: [
              GoRoute(
                path: ':id',
                builder: (context, state) => CameraDetailPage(
                  cameraId: state.pathParameters['id']!,
                ),
              ),
            ],
          ),
          GoRoute(
            path: '/incidents',
            builder: (context, state) => const IncidentListPage(),
            routes: [
              GoRoute(
                path: ':id',
                builder: (context, state) => IncidentDetailPage(
                  incidentId: state.pathParameters['id']!,
                ),
              ),
            ],
          ),
          GoRoute(
            path: '/notifications',
            builder: (context, state) => const NotificationListPage(),
          ),
          GoRoute(
            path: '/users',
            builder: (context, state) => const UserListPage(),
          ),
          GoRoute(
            path: '/settings',
            builder: (context, state) => const SettingsPage(),
          ),
        ],
      ),
    ],
    errorBuilder: (context, state) => Scaffold(
      body: Center(
        child: Text('Page not found: ${state.error}'),
      ),
    ),
  );
});
