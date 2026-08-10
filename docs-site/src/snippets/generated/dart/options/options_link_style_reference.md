```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"link_style":"Reference"}');
  final result = await H2mBridge.convert('<p><a href=\'https://example.com\'>Example</a> and <a href=\'https://other.com\'>Other</a></p>', options: _options);
}

```
