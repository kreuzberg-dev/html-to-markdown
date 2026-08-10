```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true}');
  final result = await H2mBridge.convert('<html><head><title>Gallery</title></head><body><img src="https://example.com/photo1.jpg" alt="Photo 1"><img src="https://example.com/photo2.png" alt="Photo 2"><img src="/local/image.webp" alt="Local image"></body></html>', options: _options);
}

```
