//! `ConversionOptions` image settings must reach the inline-image collector.
//!
//! `InlineImageConfig::new` seeds its own defaults, and two of them are the INVERSE of the
//! documented `ConversionOptions` defaults. When the public options are not forwarded, every
//! caller silently gets `capture_svg` and `infer_dimensions` flipped and `max_image_size`
//! ignored -- with no error and no warning.

#![cfg(feature = "inline-images")]

use html_to_markdown_rs::{ConversionOptions, convert};

const SVG_HTML: &str = r#"<p>text</p><svg width="10" height="10"><rect width="10" height="10"/></svg>"#;

#[test]
fn capture_svg_false_is_honoured_and_is_the_default() {
    let options = ConversionOptions {
        extract_images: true,
        ..ConversionOptions::default()
    };
    assert!(
        !options.capture_svg,
        "precondition: the documented default for capture_svg is false"
    );

    let result = convert(SVG_HTML, Some(options)).expect("conversion failed");

    assert!(
        result.images.is_empty(),
        "capture_svg defaults to false, so the inline <svg> must not be captured; got {} image(s)",
        result.images.len()
    );
}

#[test]
fn capture_svg_true_is_honoured_when_opted_in() {
    let options = ConversionOptions {
        extract_images: true,
        capture_svg: true,
        ..ConversionOptions::default()
    };

    let result = convert(SVG_HTML, Some(options)).expect("conversion failed");

    assert_eq!(
        result.images.len(),
        1,
        "capture_svg = true must capture the inline <svg>"
    );
}
