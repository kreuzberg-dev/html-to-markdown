---
id: fixture_dart_semantic_abbr
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
  final result = await H2mBridge.convert('<p>The <abbr title="World Wide Web">WWW</abbr> is global.</p>', options: _options);
}

```
