---
id: fixture_dart_metadata_link_type_email_classified
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true}');
  final result = await H2mBridge.convert('<p>Contact <a href="mailto:hello@example.com">us</a> directly.</p>', options: _options);
}

```
