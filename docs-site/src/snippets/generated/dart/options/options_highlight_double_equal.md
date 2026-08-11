---
id: fixture_dart_options_highlight_double_equal
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"highlight_style":"DoubleEqual"}');
  final result = await H2mBridge.convert('<p>Text with <mark>highlighted</mark> here.</p>', options: _options);
}

```
