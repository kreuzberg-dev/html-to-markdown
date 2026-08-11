---
id: fixture_dart_options_exclude_selectors_plain_text_mode
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"exclude_selectors":[".nav"],"output_format":"Plain"}');
  final result = await H2mBridge.convert('<body><div class="nav">Navigation</div><p>Article body</p></body>', options: _options);
}

```
