---
id: fixture_dart_options_highlight_none
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"highlight_style":"None"}');
  final result = await H2mBridge.convert('<p>Text with <mark>plain</mark> content.</p>', options: _options);
}

```
