class Evidence {
  final String id;
  final String incidentId;
  final String fileName;
  final String fileType;
  final int fileSize;
  final String fileUrl;
  final String? thumbnailUrl;
  final String uploadedBy;
  final DateTime uploadedAt;
  final DateTime createdAt;

  const Evidence({
    required this.id,
    required this.incidentId,
    required this.fileName,
    required this.fileType,
    required this.fileSize,
    required this.fileUrl,
    this.thumbnailUrl,
    required this.uploadedBy,
    required this.uploadedAt,
    required this.createdAt,
  });

  String get fileSizeFormatted {
    if (fileSize < 1024) return '$fileSize B';
    if (fileSize < 1024 * 1024) return '${(fileSize / 1024).toStringAsFixed(1)} KB';
    return '${(fileSize / (1024 * 1024)).toStringAsFixed(1)} MB';
  }
}
