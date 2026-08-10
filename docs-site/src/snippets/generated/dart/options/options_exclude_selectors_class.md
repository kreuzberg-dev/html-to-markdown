```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"exclude_selectors":[".cookie-banner"]}');
  final result = await H2mBridge.convert('<body><div class="cookie-banner">Accept cookies</div><p>Main content</p></body>', options: _options);
}

```
