```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"include_document_structure":true}');
  final result = await H2mBridge.convert('<table><tr><th>A</th></tr><tr><td>1</td></tr></table><p>Between</p><table><tr><th>B</th></tr><tr><td>2</td></tr></table>', options: _options);
}

```
