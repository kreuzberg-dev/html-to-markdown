---
id: fixture_dart_blockquote_with_list
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
  final result = await H2mBridge.convert('<blockquote><p>Quote intro:</p><ul><li>Point one</li><li>Point two</li></ul></blockquote>', options: _options);
}

```
