```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<p>Before SVG.</p><svg xmlns="http://www.w3.org/2000/svg"><script>alert(\'svg-xss\')</script><text>SVG text</text></svg><p>After SVG.</p>', options: _options);
}

```
