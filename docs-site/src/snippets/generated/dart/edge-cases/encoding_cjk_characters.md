---
id: fixture_dart_encoding_cjk_characters
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
  final result = await H2mBridge.convert('<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>', options: _options);
}

```
