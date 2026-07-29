import 'package:intl/intl.dart';

class AppDateUtils {
  static String format(String dateStr, {String format = 'yyyy-MM-dd'}) {
    try {
      final date = DateTime.parse(dateStr);
      return DateFormat(format).format(date);
    } catch (_) {
      return dateStr;
    }
  }

  static String timeAgo(DateTime dateTime) {
    final now = DateTime.now();
    final diff = now.difference(dateTime);

    if (diff.inSeconds < 60) {
      return '${diff.inSeconds}s ago';
    } else if (diff.inMinutes < 60) {
      return '${diff.inMinutes}m ago';
    } else if (diff.inHours < 24) {
      return '${diff.inHours}h ago';
    } else if (diff.inDays < 7) {
      return '${diff.inDays}d ago';
    } else {
      return DateFormat('MMM d, yyyy').format(dateTime);
    }
  }

  static String relativeTime(String isoDate) {
    try {
      final date = DateTime.parse(isoDate);
      return timeAgo(date);
    } catch (_) {
      return isoDate;
    }
  }
}
