---
id: fixture_dart_options_escape_underscores
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"escape_underscores":true}');
  final result = await H2mBridge.convert('<p>The variable_name is defined.</p>', options: _options);
}

```
