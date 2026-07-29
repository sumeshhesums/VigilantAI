class User {
  final String id;
  final String email;
  final String firstName;
  final String lastName;
  final String role;
  final bool enabled;
  final DateTime createdAt;
  final DateTime? updatedAt;

  const User({
    required this.id,
    required this.email,
    required this.firstName,
    required this.lastName,
    required this.role,
    required this.enabled,
    required this.createdAt,
    this.updatedAt,
  });

  String get fullName => '$firstName $lastName';
}
