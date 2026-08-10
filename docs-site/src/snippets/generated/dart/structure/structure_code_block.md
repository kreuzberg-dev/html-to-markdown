```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"include_document_structure":true}');
  final result = await H2mBridge.convert('<p>Example code:</p><pre><code class="language-rust">fn main() { println!("Hello"); }</code></pre>', options: _options);
}

```
