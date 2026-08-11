---
id: fixture_dart_blockquote_nested
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
  final result = await H2mBridge.convert('<blockquote><p>Outer quote.</p><blockquote><p>Inner quote.</p></blockquote></blockquote>', options: _options);
}

```
