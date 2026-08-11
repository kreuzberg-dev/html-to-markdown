---
id: fixture_dart_ordered_list
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
  final result = await H2mBridge.convert('<ol><li>First</li><li>Second</li><li>Third</li></ol>', options: _options);
}

```
