```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"include_document_structure":true}');
  final result = await H2mBridge.convert('<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>', options: _options);
}

```
