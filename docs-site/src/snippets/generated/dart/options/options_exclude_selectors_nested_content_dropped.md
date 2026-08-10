```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"exclude_selectors":[".sidebar"]}');
  final result = await H2mBridge.convert('<body><aside class="sidebar"><h2>Related</h2><p>Sidebar text</p></aside><main><p>Main text</p></main></body>', options: _options);
}

```
