---
id: fixture_dart_image_with_title
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
  final result = await H2mBridge.convert('<img src="chart.png" alt="Sales chart" title="Q3 Sales">', options: _options);
}

```
