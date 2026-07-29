enum LogLevel { debug, info, warning, error }

class Logger {
  static bool _enabled = true;

  static void init({bool enabled = true}) {
    _enabled = enabled;
  }

  static void debug(String message) {
    _log(LogLevel.debug, message);
  }

  static void info(String message) {
    _log(LogLevel.info, message);
  }

  static void warning(String message) {
    _log(LogLevel.warning, message);
  }

  static void error(String message, [Object? error, StackTrace? stackTrace]) {
    _log(LogLevel.error, message, error, stackTrace);
  }

  static void _log(LogLevel level, String message, [Object? error, StackTrace? stackTrace]) {
    if (!_enabled) return;
    final prefix = '[${level.name.toUpperCase()}]';
    final log = '$prefix $message';
    if (error != null) {
      final errorLog = '$log\nError: $error';
      if (stackTrace != null) {
        // ignore: avoid_print
        print('$errorLog\n$stackTrace');
      } else {
        // ignore: avoid_print
        print(errorLog);
      }
    } else {
      // ignore: avoid_print
      print(log);
    }
  }
}
