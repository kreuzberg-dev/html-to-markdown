---
id: fixture_dart_blockquote_code_block_indentation_preserved
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
  final result = await H2mBridge.convert('<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>', options: _options);
}

```
