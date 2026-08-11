---
id: fixture_dart_options_preprocessing_remove_forms
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"remove_forms":true}}');
  final result = await H2mBridge.convert('<p>Before</p><form><input type=\'text\'/><button>Submit</button></form><p>After</p>', options: _options);
}

```
