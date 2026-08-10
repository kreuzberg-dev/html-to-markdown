```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true}');
  final result = await H2mBridge.convert('<html><head><title>Test Page</title><meta name=\'description\' content=\'A test page\'></head><body><p>Content</p></body></html>', options: _options);
}

```
