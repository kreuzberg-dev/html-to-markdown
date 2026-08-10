```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true}');
  final result = await H2mBridge.convert('<html><head><title>Page</title><meta name="description" content="This is the page description."></head><body><p>Content</p></body></html>', options: _options);
}

```
