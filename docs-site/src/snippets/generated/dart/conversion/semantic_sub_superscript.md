---
id: fixture_dart_semantic_sub_superscript
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
  final result = await H2mBridge.convert('<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>', options: _options);
}

```
