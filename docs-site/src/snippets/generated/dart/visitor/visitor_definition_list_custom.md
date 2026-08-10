```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _visitor = await createHtmlVisitor(
        visitText: (ctx, text) async => VisitResult.continue_(),
        visitElementStart: (ctx) async => VisitResult.continue_(),
        visitElementEnd: (ctx, output) async => VisitResult.continue_(),
        visitLink: (ctx, href, text, title) async => VisitResult.continue_(),
        visitImage: (ctx, src, alt, title) async => VisitResult.continue_(),
        visitHeading: (ctx, level, text, id) async => VisitResult.continue_(),
        visitCodeBlock: (ctx, lang, code) async => VisitResult.continue_(),
        visitCodeInline: (ctx, code) async => VisitResult.continue_(),
        visitListItem: (ctx, ordered, marker, text) async => VisitResult.continue_(),
        visitListStart: (ctx, ordered) async => VisitResult.continue_(),
        visitListEnd: (ctx, ordered, output) async => VisitResult.continue_(),
        visitTableStart: (ctx) async => VisitResult.continue_(),
        visitTableRow: (ctx, cells, isHeader) async => VisitResult.continue_(),
        visitTableEnd: (ctx, output) async => VisitResult.continue_(),
        visitBlockquote: (ctx, content, depth) async => VisitResult.continue_(),
        visitStrong: (ctx, text) async => VisitResult.continue_(),
        visitEmphasis: (ctx, text) async => VisitResult.continue_(),
        visitStrikethrough: (ctx, text) async => VisitResult.continue_(),
        visitUnderline: (ctx, text) async => VisitResult.continue_(),
        visitSubscript: (ctx, text) async => VisitResult.continue_(),
        visitSuperscript: (ctx, text) async => VisitResult.continue_(),
        visitMark: (ctx, text) async => VisitResult.continue_(),
        visitLineBreak: (ctx) async => VisitResult.continue_(),
        visitHorizontalRule: (ctx) async => VisitResult.continue_(),
        visitCustomElement: (ctx, tagName, html) async => VisitResult.continue_(),
        visitDefinitionListStart: (ctx) async => VisitResult.continue_(),
        visitDefinitionTerm: (ctx, text) async => VisitResult.custom(field0: '**${text}**'),
        visitDefinitionDescription: (ctx, text) async => VisitResult.continue_(),
        visitDefinitionListEnd: (ctx, text) async => VisitResult.continue_(),
        visitForm: (ctx, action, method) async => VisitResult.continue_(),
        visitInput: (ctx, inputType, name, value) async => VisitResult.continue_(),
        visitButton: (ctx, text) async => VisitResult.continue_(),
        visitAudio: (ctx, src) async => VisitResult.continue_(),
        visitVideo: (ctx, src) async => VisitResult.continue_(),
        visitIframe: (ctx, src) async => VisitResult.continue_(),
        visitDetails: (ctx, open) async => VisitResult.continue_(),
        visitSummary: (ctx, text) async => VisitResult.continue_(),
        visitFigureStart: (ctx) async => VisitResult.continue_(),
        visitFigcaption: (ctx, text) async => VisitResult.continue_(),
        visitFigureEnd: (ctx, output) async => VisitResult.continue_()
      );
  final _options = await createConversionOptionsFromJsonWithVisitor(visitor: _visitor, json: '{}');
  final result = await H2mBridge.convert('<dl><dt>HTML</dt><dd>HyperText Markup Language</dd><dt>CSS</dt><dd>Cascading Style Sheets</dd></dl>', options: _options);
}

```
