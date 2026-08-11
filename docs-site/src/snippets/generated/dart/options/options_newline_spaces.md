---
id: fixture_dart_options_newline_spaces
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"newline_style":"Spaces"}');
  final result = await H2mBridge.convert('<p>First<br>Second</p>', options: _options);
}

```
