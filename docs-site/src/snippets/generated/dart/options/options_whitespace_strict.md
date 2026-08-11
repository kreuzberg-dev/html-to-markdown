---
id: fixture_dart_options_whitespace_strict
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"whitespace_mode":"Strict"}');
  final result = await H2mBridge.convert('<p>Preserved   spacing.</p>', options: _options);
}

```
