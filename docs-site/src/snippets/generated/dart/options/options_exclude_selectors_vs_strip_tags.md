---
id: fixture_dart_options_exclude_selectors_vs_strip_tags
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"exclude_selectors":[".wrapper"]}');
  final result = await H2mBridge.convert('<body><div class="wrapper"><p>Inner paragraph</p></div><p>Outer text</p></body>', options: _options);
}

```
