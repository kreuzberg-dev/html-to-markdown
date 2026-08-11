---
id: fixture_dart_options_wrap_enabled
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"wrap":true,"wrap_width":40}');
  final result = await H2mBridge.convert('<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>', options: _options);
}

```
