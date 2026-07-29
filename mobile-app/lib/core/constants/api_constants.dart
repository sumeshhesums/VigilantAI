class ApiConstants {
  static const String auth = '/auth';
  static const String login = '/auth/login';
  static const String register = '/auth/register';
  static const String refreshToken = '/auth/refresh';
  static const String logout = '/auth/logout';
  static const String me = '/auth/me';

  static const String cameras = '/cameras';
  static const String cameraById = '/cameras/';

  static const String incidents = '/incidents';
  static const String incidentById = '/incidents/';

  static const String evidence = '/evidence';
  static const String evidenceById = '/evidence/';

  static const String notifications = '/notifications';
  static const String notificationById = '/notifications/';
  static const String notificationsMarkRead = '/notifications/';
  static const String notificationsMarkAllRead = '/notifications/mark-all-read';

  static const String users = '/users';
  static const String userById = '/users/';

  static const String roles = '/roles';
  static const String roleById = '/roles/';

  static const String dashboard = '/dashboard';
  static const String dashboardKpis = '/dashboard/kpis';
  static const String dashboardLiveStats = '/dashboard/live-stats';
  static const String dashboardAlertTrends = '/dashboard/alert-trends';
  static const String dashboardIncidentsSummary = '/dashboard/incidents-summary';
}
