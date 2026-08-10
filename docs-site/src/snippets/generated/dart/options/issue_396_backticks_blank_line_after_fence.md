```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"code_block_style":"Backticks"}');
  final result = await H2mBridge.convert('<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>', options: _options);
}

```
