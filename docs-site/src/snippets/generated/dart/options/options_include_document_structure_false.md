```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"include_document_structure":false}');
  final result = await H2mBridge.convert('<article><h1>Heading</h1><p>Paragraph body.</p></article>', options: _options);
}

```
