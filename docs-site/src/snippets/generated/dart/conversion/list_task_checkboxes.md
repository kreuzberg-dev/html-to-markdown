---
id: fixture_dart_list_task_checkboxes
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
  final result = await H2mBridge.convert('<ul><li><input type="checkbox" checked> Done task</li><li><input type="checkbox"> Pending task</li></ul>', options: _options);
}

```
