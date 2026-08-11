---
id: fixture_dart_conversion_autolink_mixed_filename_and_url
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
  final result = await H2mBridge.convert('<a href="foobar.png">foobar.png</a> <a href="https://www.heise.de">https://www.heise.de</a>', options: _options);
}

```
