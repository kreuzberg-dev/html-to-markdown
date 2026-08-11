---
id: fixture_dart_semantic_details_summary
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
  final result = await H2mBridge.convert('<details><summary>Click to expand</summary><p>Hidden content here.</p></details>', options: _options);
}

```
