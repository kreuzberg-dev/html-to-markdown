---
id: fixture_dart_result_warnings_empty_for_malformed_html
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
  final result = await H2mBridge.convert('<p>Unclosed paragraph<div>Mixed nesting</p></div>', options: _options);
}

```
