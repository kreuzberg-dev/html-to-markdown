---
id: fixture_dart_unordered_list
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
  final result = await H2mBridge.convert('<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>', options: _options);
}

```
