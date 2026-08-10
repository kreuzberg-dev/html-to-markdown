```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"code_block_style":"Tildes"}');
  final result = await H2mBridge.convert('<pre><code>some code</code></pre>', options: _options);
}

```
