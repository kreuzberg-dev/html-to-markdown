// swift-tools-version: 6.0
// The first-party dependency pin below is managed by alef (sync.text_replacements); do not edit it by hand.
// alef:hash:84dcb57196bf35ece701ab60981f40f9aa8dd250604ea042ac65b2a24570dc0e
import PackageDescription

let package = Package(
  name: "E2eSwift",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  dependencies: [
    .package(url: "https://github.com/xberg-io/html-to-markdown", branch: "release/swift/3.12.1"),
  ],
  targets: [
    .testTarget(
      name: "HtmlToMarkdownE2ETests",
      dependencies: [.product(name: "HtmlToMarkdown", package: "html-to-markdown")]
    ),
  ]
)
