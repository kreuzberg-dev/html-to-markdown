---
id: fixture_dart_code_with_backticks_in_content
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<p>Use <code>`backtick` here</code> carefully.</p>', options: _options);
}

```
