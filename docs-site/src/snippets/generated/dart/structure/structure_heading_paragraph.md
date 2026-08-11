---
id: fixture_dart_structure_heading_paragraph
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
  final result = await H2mBridge.convert('<h1>Title</h1><p>A paragraph of text.</p>', options: _options);
}

```
