---
id: fixture_dart_options_escape_misc
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"escape_misc":true}');
  final result = await H2mBridge.convert('<p>Use # and | and ~ in text.</p>', options: _options);
}

```
