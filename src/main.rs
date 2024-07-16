use minix2_rs::interpreter::vm_interpret;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    vm_interpret(args);
}
