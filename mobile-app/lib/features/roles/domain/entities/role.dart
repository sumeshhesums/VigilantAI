class Role {
  final String id;
  final String name;
  final String description;
  final List<String> permissions;
  final DateTime createdAt;
  final DateTime? updatedAt;

  const Role({
    required this.id,
    required this.name,
    required this.description,
    required this.permissions,
    required this.createdAt,
    this.updatedAt,
  });
}
