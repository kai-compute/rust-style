use std::cell::Cell;
use std::error::Error;
use std::io::BufReader;
use std::rc::Rc;

use super::*;

#[derive(Default)]
struct ShortWriter {
    bytes: Vec<u8>,
    flushes: usize,
    fail_flush: bool,
}

impl Write for ShortWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let count = bytes.len().min(2);
        self.bytes.extend_from_slice(&bytes[..count]);
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushes += 1;
        if self.fail_flush {
            Err(io::ErrorKind::Other.into())
        } else {
            Ok(())
        }
    }
}

#[test]
fn empty_input_flushes_and_returns_an_empty_summary() {
    let mut output = ShortWriter::default();
    assert_eq!(
        transform(Cursor::new(b""), &mut output).unwrap(),
        Summary::default()
    );
    assert_eq!(output.flushes, 1);
    assert!(output.bytes.is_empty());
}

#[test]
fn chunk_boundaries_preserve_records_and_utf8_failure_class() {
    for size in 1..=16 {
        let mut output = ShortWriter::default();
        let input = BufReader::with_capacity(size, Cursor::new(b"a,002\r\nb_1,3\nc-2,4"));
        assert_eq!(
            transform(input, &mut output).unwrap(),
            Summary {
                records: 3,
                units: 9
            }
        );
        assert_eq!(output.bytes, b"a=2\nb_1=3\nc-2=4\n");
        assert_eq!(output.flushes, 1);

        let input = BufReader::with_capacity(size, Cursor::new(b"a,1\n\xce\xbb,2\n"));
        let mut output = Vec::new();
        let error = transform(input, &mut output).unwrap_err();
        assert!(matches!(
            error,
            PipelineError::Malformed {
                line: 2,
                cause: RecordError::Label
            }
        ));
        assert!(error.source().unwrap().is::<RecordError>());
        assert_eq!(output, b"a=1\n");
    }
}

#[test]
fn record_limit_includes_each_terminator() {
    for terminator in ["", "\n", "\r\n"] {
        let record = format!(
            "a,{}1{terminator}",
            "0".repeat(MAX_RECORD_BYTES - 3 - terminator.len())
        );
        assert_eq!(record.len(), MAX_RECORD_BYTES);
        let mut output = Vec::new();
        assert_eq!(
            transform(Cursor::new(record), &mut output).unwrap().units,
            1
        );
        assert_eq!(output, b"a=1\n");

        let oversized = format!(
            "a,{}1{terminator}",
            "0".repeat(MAX_RECORD_BYTES - 2 - terminator.len())
        );
        let mut output = Vec::new();
        assert!(matches!(
            transform(Cursor::new(oversized), &mut output),
            Err(PipelineError::RecordTooLong { line: 1 })
        ));
        assert!(output.is_empty());
    }
}

#[test]
fn grammar_rejects_signs_extra_fields_and_invalid_labels() {
    for (input, expected) in [
        ("", RecordError::Fields),
        ("A,1", RecordError::Label),
        ("a,+1", RecordError::Count),
        ("a,4294967296", RecordError::Count),
        ("a,1,2", RecordError::Count),
        ("a,1\r", RecordError::Count),
    ] {
        assert_eq!(parse_record(input), Err(expected), "input: {input:?}");
    }
    assert_eq!(
        parse_record(&format!("{},1", "a".repeat(33))),
        Err(RecordError::Label)
    );
    assert_eq!(parse_record("a,4294967295").unwrap().count.get(), u32::MAX);
}

struct FailingReader {
    prefix: Cursor<Vec<u8>>,
    dropped: Rc<Cell<bool>>,
}

impl Read for FailingReader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        if self.prefix.position() == self.prefix.get_ref().len() as u64 {
            return Err(io::ErrorKind::ConnectionReset.into());
        }
        self.prefix.read(buffer)
    }
}

impl Drop for FailingReader {
    fn drop(&mut self) {
        self.dropped.set(true);
    }
}

#[test]
fn read_failure_keeps_completed_output_and_releases_the_owned_reader() {
    let dropped = Rc::new(Cell::new(false));
    let input = BufReader::new(FailingReader {
        prefix: Cursor::new(b"a,1\n".to_vec()),
        dropped: Rc::clone(&dropped),
    });
    let mut output = ShortWriter::default();
    let error = transform(input, &mut output).unwrap_err();
    assert!(
        matches!(&error, PipelineError::Read(cause) if cause.kind() == io::ErrorKind::ConnectionReset)
    );
    assert!(error.source().unwrap().is::<io::Error>());
    assert_eq!(output.bytes, b"a=1\n");
    assert_eq!(output.flushes, 0);
    assert!(dropped.get());
}

#[test]
fn partial_write_errors_retain_the_original_cause() {
    let mut output = FailAfter {
        limit: 3,
        bytes: Vec::new(),
    };
    let error = transform(Cursor::new(b"alpha,1\n"), &mut output).unwrap_err();
    assert!(
        matches!(&error, PipelineError::Write(cause) if cause.kind() == io::ErrorKind::BrokenPipe)
    );
    assert!(error.source().unwrap().is::<io::Error>());
    assert_eq!(output.bytes, b"alp");
}

#[test]
fn final_flush_failure_is_not_reported_as_success() {
    let mut output = ShortWriter {
        fail_flush: true,
        ..ShortWriter::default()
    };
    let error = transform(Cursor::new(b"a,1\n"), &mut output).unwrap_err();
    assert!(matches!(&error, PipelineError::Flush(cause) if cause.kind() == io::ErrorKind::Other));
    assert!(error.source().unwrap().is::<io::Error>());
    assert_eq!(output.bytes, b"a=1\n");
    assert_eq!(output.flushes, 1);
}
