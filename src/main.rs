use env_logger;
use minix2_rs::interpreter::Interpretable;
use minix2_rs::minix::Program;

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        // .filter_level(log::LevelFilter::Trace)
        .filter_level(log::LevelFilter::Info)
        .init();

    let args: Vec<String> = std::env::args().collect();

    // Args validation
    if args.len() < 2 {
        println!("Usage: {} <binary file>", args[0]);
        return;
    }

    // Open file
    let file = std::fs::File::open(&args[1]).unwrap();
    let program = Program::from_file(file).unwrap();

    // Disassembler
    // let disassembled = program.disassemble().unwrap();

    // Interpreter
    program.interpret();
}
