```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"enabled":false}}');
  final result = await H2mBridge.convert('<nav>NavSection</nav><p>Paragraph</p>', options: _options);
}

```
