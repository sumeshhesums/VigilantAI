class User {
  final String id;
  final String email;
  final String firstName;
  final String lastName;
  final String role;
  final DateTime createdAt;

  const User({
    required this.id,
    required this.email,
    required this.firstName,
    required this.lastName,
    required this.role,
    required this.createdAt,
  });

  String get fullName => '$firstName $lastName';
  bool get isAdmin => role == 'system_admin' || role == 'security_admin';
}
