//! Boundary checks for the implementations extracted from README.md.

use std::io::{self, Write};

use frame_codec::b02::find_value;
use frame_codec::b03::parse_counts;
use frame_codec::b04::write_frame;

#[test]
fn lookup_borrows_from_entries_after_the_key_is_dropped() {
    let entries = vec![("name".to_owned(), "value".to_owned())];
    let value = {
        let key = "name".to_owned();
        find_value(&entries, &key).expect("the entry exists")
    };
    assert_eq!(value, "value");
    assert_eq!(value.as_ptr(), entries[0].1.as_ptr());
}

#[test]
fn lookup_returns_the_first_exact_match_or_none() {
    let entries = vec![
        ("name".to_owned(), "first".to_owned()),
        ("name".to_owned(), "second".to_owned()),
    ];
    assert_eq!(find_value(&entries, "name"), Some("first"));
    assert_eq!(find_value(&entries, "NAME"), None);
    assert_eq!(find_value(&[], "name"), None);
}

#[test]
fn parsing_accepts_empty_input_and_rejects_overflow() {
    assert_eq!(parse_counts(&[]), Ok(vec![]));
    let error = parse_counts(&["4294967296"]).expect_err("the value exceeds u32::MAX");
    assert_eq!(error.kind(), &std::num::IntErrorKind::PosOverflow);
}

#[test]
fn oversized_payload_is_rejected_before_any_write() {
    let mut output = Vec::new();
    let error = write_frame(&mut output, &vec![0; 1_048_577])
        .expect_err("the payload exceeds the frame limit");
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert!(output.is_empty());
}

#[test]
fn empty_payload_still_writes_a_header() -> io::Result<()> {
    let mut output = Vec::new();
    write_frame(&mut output, b"")?;
    assert_eq!(output, [0, 0, 0, 0]);
    Ok(())
}

#[test]
fn writer_failure_preserves_partial_effects() {
    let mut output = [0; 5];
    let error = write_frame(&mut output[..], b"abc")
        .expect_err("the destination cannot hold the full frame");
    assert_eq!(error.kind(), io::ErrorKind::WriteZero);
    assert_eq!(&output, b"\0\0\0\x03a");
}

#[derive(Default)]
struct ShortWriter {
    output: Vec<u8>,
    interrupted: bool,
    flushed: bool,
}

impl Write for ShortWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if !self.interrupted {
            self.interrupted = true;
            return Err(io::ErrorKind::Interrupted.into());
        }
        let count = bytes.len().min(2);
        self.output.extend_from_slice(&bytes[..count]);
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushed = true;
        Ok(())
    }
}

#[test]
fn short_and_interrupted_writes_complete_without_flushing() -> io::Result<()> {
    let mut writer = ShortWriter::default();
    write_frame(&mut writer, b"abc")?;
    assert_eq!(writer.output, b"\0\0\0\x03abc");
    assert!(!writer.flushed);
    Ok(())
}
