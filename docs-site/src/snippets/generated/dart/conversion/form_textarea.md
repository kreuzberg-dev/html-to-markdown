---
id: fixture_dart_form_textarea
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"remove_forms":false}}');
  final result = await H2mBridge.convert('<form><label>Message:</label><textarea>Default text content</textarea></form>', options: _options);
}

```
