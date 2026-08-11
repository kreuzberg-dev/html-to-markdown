---
id: fixture_dart_options_code_block_tildes_style
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"code_block_style":"Tildes"}');
  final result = await H2mBridge.convert('<pre><code>some code</code></pre>', options: _options);
}

```
