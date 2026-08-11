---
id: fixture_dart_xss_script_tag_stripped
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
  final result = await H2mBridge.convert('<p>Safe content.</p><script>alert(\'xss\')</script><p>More safe content.</p>', options: _options);
}

```
