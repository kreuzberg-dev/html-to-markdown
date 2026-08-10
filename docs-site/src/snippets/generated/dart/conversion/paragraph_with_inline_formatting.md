```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<p>This has <strong>bold</strong>, <em>italic</em>, and a <a href="https://example.com">link</a>.</p>', options: _options);
}

```
