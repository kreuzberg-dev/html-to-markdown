```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"list_indent_type":"Tabs"}');
  final result = await H2mBridge.convert('<ul><li>Parent<ul><li>Child</li></ul></li></ul>', options: _options);
}

```
