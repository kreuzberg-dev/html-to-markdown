---
id: fixture_dart_blockquote_nested_list_indentation_preserved
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
  final result = await H2mBridge.convert('<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>', options: _options);
}

```
