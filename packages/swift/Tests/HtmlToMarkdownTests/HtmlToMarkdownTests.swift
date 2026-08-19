import XCTest

@testable import HtmlToMarkdown

final class HtmlToMarkdownTests: XCTestCase {
  /// Round-trips the generated `ImageDimensions` DTO through `JSONEncoder`/`JSONDecoder`,
  /// so a broken `Codable` conformance or a field that silently stops encoding fails
  /// `swift test` immediately instead of shipping green with a suite that asserts
  /// nothing about the generated API. Create-only scaffold seed. ~keep
  func testImageDimensionsRoundTripsThroughJSON() throws {
    let original = ImageDimensions(width: 1, height: 1)
    let data = try JSONEncoder().encode(original)
    let decoded = try JSONDecoder().decode(ImageDimensions.self, from: data)
    XCTAssertEqual(decoded, original)
  }
}
