---
id: fixture_dart_options_preserve_tags_iframe
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preserve_tags":["iframe"]}');
  final result = await H2mBridge.convert('<p>Before</p><iframe src=\'video.html\' width=\'560\'></iframe><p>After</p>', options: _options);
}

```
