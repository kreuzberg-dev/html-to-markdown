---
id: fixture_dart_structure_list
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"include_document_structure":true}');
  final result = await H2mBridge.convert('<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>', options: _options);
}

```
