```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"list_indent_width":4}');
  final result = await H2mBridge.convert('<ul><li>Outer<ul><li>Inner</li></ul></li></ul>', options: _options);
}

```
