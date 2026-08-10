```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<figure><img src="sunset.jpg" alt="A sunset"><figcaption>Beautiful sunset over the ocean</figcaption></figure>', options: _options);
}

```
