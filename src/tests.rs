use pretty_assertions::assert_eq;
use std::fs;

use crate::disassembler::minix2_disassemble;

fn assert_disassemble(file: &str) {
    let args = vec![
        "minix2_rs".to_string(),
        format!("./tests_data/{}.out", file),
    ];
    let result = minix2_disassemble(args).unwrap();
    let expected = fs::read_to_string(format!("./tests_data/{}.expected", file)).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_asem_1() {
    assert_disassemble("asem/1.s");
}

#[test]
fn test_asem_2() {
    assert_disassemble("asem/2.s");
}

#[test]
fn test_asem_3() {
    assert_disassemble("asem/3.s");
}

#[test]
fn test_asem_4() {
    assert_disassemble("asem/4.s");
}

#[test]
fn test_c_1() {
    assert_disassemble("1.c");
}

#[test]
fn test_c_2() {
    assert_disassemble("2.c");
}

#[test]
fn test_c_3() {
    assert_disassemble("3.c");
}
