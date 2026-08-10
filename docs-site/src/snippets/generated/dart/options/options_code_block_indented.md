```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"code_block_style":"Indented"}');
  final result = await H2mBridge.convert('<pre><code>print(\'hello\')</code></pre>', options: _options);
}

```
