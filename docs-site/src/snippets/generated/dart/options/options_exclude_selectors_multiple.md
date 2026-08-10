```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"exclude_selectors":[".nav","footer"]}');
  final result = await H2mBridge.convert('<body><nav class="nav">Menu</nav><p>Content</p><footer>Footer</footer></body>', options: _options);
}

```
