```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"code_block_style":"Backticks"}');
  final result = await H2mBridge.convert('<pre><code class="language-js">console.log(\'hi\');</code></pre>', options: _options);
}

```
