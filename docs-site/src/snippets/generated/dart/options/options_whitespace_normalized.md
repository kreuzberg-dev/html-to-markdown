---
id: fixture_dart_options_whitespace_normalized
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"whitespace_mode":"Normalized"}');
  final result = await H2mBridge.convert('<p>Text   with    extra   spaces.</p>', options: _options);
}

```
