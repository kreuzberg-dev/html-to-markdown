---
id: fixture_dart_options_code_block_indented
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"code_block_style":"Indented"}');
  final result = await H2mBridge.convert('<pre><code>print(\'hello\')</code></pre>', options: _options);
}

```
