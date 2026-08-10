```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"convert_as_inline":true}');
  final result = await H2mBridge.convert('<p>One</p><p>Two</p>', options: _options);
}

```
