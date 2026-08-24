import 'package:test/test.dart';
import 'package:h2m/h2m.dart' as html_to_markdown_rs;

void main() {
  test('ImageDimensions equality holds for identical field values', () {
    // Literal-constructs the generated `ImageDimensions` DTO twice with identical field
    // values and compares them for equality, so a constructor that drops/renames a
    // field, or generated equality that stops being field-based, fails `dart test`
    // immediately instead of shipping green with a suite that asserts nothing about
    // the generated API. Create-only scaffold seed. ~keep
    final a = html_to_markdown_rs.ImageDimensions(width: 1, height: 1);
    final b = html_to_markdown_rs.ImageDimensions(width: 1, height: 1);
    expect(a, equals(b));
  });
}
