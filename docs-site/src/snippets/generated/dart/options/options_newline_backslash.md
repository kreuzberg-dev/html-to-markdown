---
id: fixture_dart_options_newline_backslash
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"newline_style":"Backslash"}');
  final result = await H2mBridge.convert('<p>Line one<br>Line two</p>', options: _options);
}

```
