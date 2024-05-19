use minix2_rs::disassembler::minix2_disassemble;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Args validation
    if args.len() < 2 {
        println!("Usage: {} <binary file>", args[0]);
        return;
    }

    match minix2_disassemble(args) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("{}", e),
    }
}
