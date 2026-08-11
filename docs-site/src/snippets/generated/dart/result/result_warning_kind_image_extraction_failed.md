---
id: fixture_dart_result_warning_kind_image_extraction_failed
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_images":true}');
  final result = await H2mBridge.convert('<p>Text<img src="data:BADMIME" alt="broken">end</p>', options: _options);
}

```
