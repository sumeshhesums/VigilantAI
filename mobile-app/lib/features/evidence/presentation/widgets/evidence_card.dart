import 'package:flutter/material.dart';

import '../../domain/entities/evidence.dart';

class EvidenceCard extends StatelessWidget {
  final Evidence evidence;
  final VoidCallback? onDelete;
  final VoidCallback? onTap;

  const EvidenceCard({
    super.key,
    required this.evidence,
    this.onDelete,
    this.onTap,
  });

  IconData _fileIcon(String fileType) {
    switch (fileType.toLowerCase()) {
      case 'image/jpeg':
      case 'image/png':
      case 'image/gif':
      case 'image':
        return Icons.image;
      case 'video/mp4':
      case 'video/mpeg':
      case 'video':
        return Icons.videocam;
      case 'application/pdf':
        return Icons.picture_as_pdf;
      default:
        return Icons.insert_drive_file;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      clipBehavior: Clip.antiAlias,
      child: InkWell(
        onTap: onTap,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (evidence.thumbnailUrl != null)
              Image.network(
                evidence.thumbnailUrl!,
                height: 120,
                width: double.infinity,
                fit: BoxFit.cover,
                errorBuilder: (_, __, ___) => _filePlaceholder(),
              )
            else
              _filePlaceholder(),
            Padding(
              padding: const EdgeInsets.all(8),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    evidence.fileName,
                    style: Theme.of(context).textTheme.bodyMedium?.copyWith(fontWeight: FontWeight.w600),
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                  ),
                  const SizedBox(height: 4),
                  Text(
                    evidence.fileSizeFormatted,
                    style: Theme.of(context).textTheme.bodySmall?.copyWith(color: Colors.grey),
                  ),
                  if (onDelete != null)
                    Align(
                      alignment: Alignment.centerRight,
                      child: IconButton(
                        icon: const Icon(Icons.delete_outline, size: 20),
                        onPressed: onDelete,
                        color: Colors.red,
                      ),
                    ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _filePlaceholder() {
    return Container(
      height: 120,
      color: Colors.grey.shade200,
      child: Center(
        child: Icon(
          _fileIcon(evidence.fileType),
          size: 48,
          color: Colors.grey.shade400,
        ),
      ),
    );
  }
}
