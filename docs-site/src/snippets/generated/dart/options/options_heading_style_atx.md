---
id: fixture_dart_options_heading_style_atx
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"heading_style":"Atx"}');
  final result = await H2mBridge.convert('<h1>Title</h1><h2>Subtitle</h2>', options: _options);
}

```
