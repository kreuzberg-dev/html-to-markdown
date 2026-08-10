```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"escape_ascii":true}');
  final result = await H2mBridge.convert('<p>Text with # hash and [brackets] and * star</p>', options: _options);
}

```
