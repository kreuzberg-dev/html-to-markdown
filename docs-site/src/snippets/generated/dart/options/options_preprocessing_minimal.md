```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"preset":"Minimal"}}');
  final result = await H2mBridge.convert('<nav>Navigation</nav><p>Content</p><footer>Footer</footer>', options: _options);
}

```
