---
id: fixture_dart_options_skip_images_true
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"skip_images":true}');
  final result = await H2mBridge.convert('<p>Before <img src=\'test.jpg\' alt=\'photo\'> After</p>', options: _options);
}

```
