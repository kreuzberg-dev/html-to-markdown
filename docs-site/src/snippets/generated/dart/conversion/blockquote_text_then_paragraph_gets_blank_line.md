---
id: fixture_dart_blockquote_text_then_paragraph_gets_blank_line
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
  final result = await H2mBridge.convert('<blockquote>Just text, then <p>a paragraph</p></blockquote>', options: _options);
}

```
