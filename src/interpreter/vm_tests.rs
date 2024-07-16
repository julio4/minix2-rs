use pretty_assertions::assert_eq;
use std::io::Write;
use std::{
    fs,
    sync::{Arc, Mutex},
};

use crate::interpreter::vm_interpret;

struct BufferWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl Write for BufferWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn assert_interpret(file: &str) {
    let mut args = vec![
        "minix2_rs".to_string(),
        format!("./tests_data/{}.out", file),
    ];
    let expected_out = fs::read_to_string(format!("./tests_data/{}.expected", file)).unwrap();
    let expected_trace = fs::read_to_string(format!("./tests_data/{}.vm_expected", file)).unwrap();

    // Capture stdout output without -m flag
    std::io::set_output_capture(Some(Default::default()));

    vm_interpret(args.clone());

    let captured = std::io::set_output_capture(None);
    let captured = captured.unwrap();
    let captured = Arc::try_unwrap(captured).unwrap();
    let captured = captured.into_inner().unwrap();
    let captured = String::from_utf8(captured).unwrap();

    assert_eq!(captured, expected_out);

    // With -m flag
    args.push("-m".to_string());

    std::io::set_output_capture(Some(Default::default()));

    vm_interpret(args);

    let captured = std::io::set_output_capture(None);
    let captured = captured.unwrap();
    let captured = Arc::try_unwrap(captured).unwrap();
    let captured = captured.into_inner().unwrap();
    let captured = String::from_utf8(captured).unwrap();

    assert_eq!(captured, expected_trace);
}

#[test]
fn test_vm_c_1() {
    assert_interpret("1.c");
}

// #[test]
// fn test_vm_c_2() {
//     assert_interpret("2.c");
// }
