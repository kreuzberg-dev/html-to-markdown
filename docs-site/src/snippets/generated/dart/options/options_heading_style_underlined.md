```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"heading_style":"Underlined"}');
  final result = await H2mBridge.convert('<h1>Main Title</h1>', options: _options);
}

```
