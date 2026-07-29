class Evidence {
  final String id;
  final String incidentId;
  final String fileName;
  final String contentType;
  final int fileSize;
  final String sha256;
  final int? width;
  final int? height;
  final DateTime createdAt;

  const Evidence({
    required this.id,
    required this.incidentId,
    required this.fileName,
    required this.contentType,
    required this.fileSize,
    required this.sha256,
    this.width,
    this.height,
    required this.createdAt,
  });

  String get fileSizeFormatted {
    if (fileSize < 1024) return '$fileSize B';
    if (fileSize < 1024 * 1024) return '${(fileSize / 1024).toStringAsFixed(1)} KB';
    return '${(fileSize / (1024 * 1024)).toStringAsFixed(1)} MB';
  }
}
