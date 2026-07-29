class User {
  final String id;
  final String email;
  final String firstName;
  final String lastName;
  final List<String> roles;
  final bool isActive;
  final DateTime createdAt;
  final DateTime? updatedAt;

  const User({
    required this.id,
    required this.email,
    required this.firstName,
    required this.lastName,
    required this.roles,
    required this.isActive,
    required this.createdAt,
    this.updatedAt,
  });

  String get fullName => '$firstName $lastName';
}
