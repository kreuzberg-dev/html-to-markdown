```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<p>Safe content.</p><script>alert(\'xss\')</script><p>More safe content.</p>', options: _options);
}

```
