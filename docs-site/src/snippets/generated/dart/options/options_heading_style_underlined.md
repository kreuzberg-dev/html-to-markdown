---
id: fixture_dart_options_heading_style_underlined
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"heading_style":"Underlined"}');
  final result = await H2mBridge.convert('<h1>Main Title</h1>', options: _options);
}

```
