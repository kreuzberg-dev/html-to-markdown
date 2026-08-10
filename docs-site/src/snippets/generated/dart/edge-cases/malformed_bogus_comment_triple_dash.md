```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<h1>One</h1>\n<!-- /// --->\n<p>Two</p>', options: _options);
}

```
