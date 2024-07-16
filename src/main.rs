// use minix2_rs::disassembler::Disassemblable;
use minix2_rs::interpreter::Interpretable;
use minix2_rs::minix::Program;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Args validation
    if args.len() < 2 {
        println!("Usage: {} <binary file> [-m]", args[0]);
        return;
    }

    // Logger
    let trace = args.len() > 2 && args[2] == "-m";
    if trace {
        std::env::set_var("RUST_LOG", "trace");
    }
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .format_level(false)
        // don't print new line
        .format(|buf, record| {
            use std::io::Write;
            write!(buf, "{}", record.args())
        })
        .filter_level(if trace {
            log::LevelFilter::Trace
        } else {
            log::LevelFilter::Info
        })
        .init();

    // Open file
    let file = std::fs::File::open(&args[1]).unwrap();
    let program = Program::from_file(file).unwrap();

    // Disassembler
    // let disassembled = program.disassemble().unwrap();
    // println!("{}", disassembled);

    // Interpreter
    program.interpret().unwrap();
}
