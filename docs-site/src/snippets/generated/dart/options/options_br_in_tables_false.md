---
id: fixture_dart_options_br_in_tables_false
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"br_in_tables":false}');
  final result = await H2mBridge.convert('<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>', options: _options);
}

```
