# minix2-rs

Rust crate for disassembling and interpreting Minix 2 binaries compiled for the 8086 CPU.
It can be used as a virtual machine for Minix 2 binaries.

## Note

This crate is for education purposes only. A partial implementation of the instruction set is provided.

## Features

- Disassembler: read a Minix 2 binary and output the sequence of CPU instructions.
- Interpreter: execute a sequence of CPU instructions and simulate the behavior of the 8086 CPU, including the stack, registers, memory and minix2 system calls.

## Cli Usage

Compile with `cargo build --release` and run the binary with the path to the minix binary as argument:

```sh
./target/release/minix2-rs path-to-binary
```

Alternatively, you can use `cargo run path-to-binary` to run it directly.

You can also use the `-m` flag to output the state of the CPU registers and memory after each instruction:

## Library Usage

Read minix binary from file:

```rust
use minix2_rs::minix::Program;

let file = std::fs::File::open(&args[1]).unwrap();
let program = Program::from_file(file).unwrap();
```

Disassemble program and output assembly code to stdout:

```rust
use minix2_rs::disassembler::Disassemblable;

let disassembled = program.disassemble().unwrap();
println!("{}", disassembled);
```

Interpret program in minix2 virtual machine environment:

```rust
use minix2_rs::interpreter::Interpretable;

program.interpret();
```

## Documentation

Use `cargo doc --open` to generate and open the documentation in your browser.

## Examples

````sh
cargo run tests_data/1.c.out
hello

```sh
cargo run tests_data/1.c.out -m
 AX   BX   CX   DX   SP   BP   SI   DI  FLAGS IP
0000 0000 0000 0000 ffda 0000 0000 0000 ---- 0000:31ed         xor bp, bp
0000 0000 0000 0000 ffda 0000 0000 0000 --Z- 0002:89e3         mov bx, sp
0000 ffda 0000 0000 ffda 0000 0000 0000 --Z- 0004:8b07         mov ax, [bx] ;[ffda]0001
0001 ffda 0000 0000 ffda 0000 0000 0000 --Z- 0006:8d5702       lea dx, [bx+2] ;[ffdc]ffe4
0001 ffda 0000 ffdc ffda 0000 0000 0000 --Z- 0009:8d4f04       lea cx, [bx+4] ;[ffde]0000
0001 ffda ffde ffdc ffda 0000 0000 0000 --Z- 000c:01c1         add cx, ax
0001 ffda ffdf ffdc ffda 0000 0000 0000 -S-- 000e:01c1         add cx, ax
0001 ffda ffe0 ffdc ffda 0000 0000 0000 -S-- 0010:bb1000       mov bx, 0010
0001 0010 ffe0 ffdc ffda 0000 0000 0000 -S-- 0013:81fb1400     cmp bx, 0014
0001 0010 ffe0 ffdc ffda 0000 0000 0000 -S-C 0017:730f         jnb 0028
0001 0010 ffe0 ffdc ffda 0000 0000 0000 -S-C 0019:f6c301       test bl, 1
0001 0010 ffe0 ffdc ffda 0000 0000 0000 --Z- 001c:750a         jne 0028
0001 0010 ffe0 ffdc ffda 0000 0000 0000 --Z- 001e:813f5353     cmp [bx], 5353 ;[0010]5353
0001 0010 ffe0 ffdc ffda 0000 0000 0000 --Z- 0022:7504         jne 0028
0001 0010 ffe0 ffdc ffda 0000 0000 0000 --Z- 0024:891e0200     mov [0002], bx ;[0002]0014
0001 0010 ffe0 ffdc ffda 0000 0000 0000 --Z- 0028:8b1e0200     mov bx, [0002] ;[0002]0010
0001 0010 ffe0 ffdc ffda 0000 0000 0000 --Z- 002c:890f         mov [bx], cx ;[0010]5353
0001 0010 ffe0 ffdc ffda 0000 0000 0000 --Z- 002e:51           push cx
0001 0010 ffe0 ffdc ffd8 0000 0000 0000 --Z- 002f:52           push dx
0001 0010 ffe0 ffdc ffd6 0000 0000 0000 --Z- 0030:50           push ax
0001 0010 ffe0 ffdc ffd4 0000 0000 0000 --Z- 0031:e80500       call 0039
0001 0010 ffe0 ffdc ffd2 0000 0000 0000 --Z- 0039:55           push bp
0001 0010 ffe0 ffdc ffd0 0000 0000 0000 --Z- 003a:89e5         mov bp, sp
0001 0010 ffe0 ffdc ffd0 ffd0 0000 0000 --Z- 003c:b80600       mov ax, 0006
0006 0010 ffe0 ffdc ffd0 ffd0 0000 0000 --Z- 003f:50           push ax
0006 0010 ffe0 ffdc ffce ffd0 0000 0000 --Z- 0040:b80400       mov ax, 0004
0004 0010 ffe0 ffdc ffce ffd0 0000 0000 --Z- 0043:50           push ax
0004 0010 ffe0 ffdc ffcc ffd0 0000 0000 --Z- 0044:b80100       mov ax, 0001
0001 0010 ffe0 ffdc ffcc ffd0 0000 0000 --Z- 0047:50           push ax
0001 0010 ffe0 ffdc ffca ffd0 0000 0000 --Z- 0048:e84100       call 008c
0001 0010 ffe0 ffdc ffc8 ffd0 0000 0000 --Z- 008c:e92100       jmp 00b0
0001 0010 ffe0 ffdc ffc8 ffd0 0000 0000 --Z- 00b0:55           push bp
0001 0010 ffe0 ffdc ffc6 ffd0 0000 0000 --Z- 00b1:89e5         mov bp, sp
0001 0010 ffe0 ffdc ffc6 ffc6 0000 0000 --Z- 00b3:83ec18       sub sp, 18
0001 0010 ffe0 ffdc ffae ffc6 0000 0000 -S-- 00b6:8b5604       mov dx, [bp+4] ;[ffca]0001
0001 0010 ffe0 0001 ffae ffc6 0000 0000 -S-- 00b9:8956ec       mov [bp-14], dx ;[ffb2]0000
0001 0010 ffe0 0001 ffae ffc6 0000 0000 -S-- 00bc:8b5608       mov dx, [bp+8] ;[ffce]0006
0001 0010 ffe0 0006 ffae ffc6 0000 0000 -S-- 00bf:8956ee       mov [bp-12], dx ;[ffb4]0000
0001 0010 ffe0 0006 ffae ffc6 0000 0000 -S-- 00c2:8b5606       mov dx, [bp+6] ;[ffcc]0004
0001 0010 ffe0 0004 ffae ffc6 0000 0000 -S-- 00c5:8956f2       mov [bp-e], dx ;[ffb8]0000
0001 0010 ffe0 0004 ffae ffc6 0000 0000 -S-- 00c8:8d46e8       lea ax, [bp-18] ;[ffae]0000
ffae 0010 ffe0 0004 ffae ffc6 0000 0000 -S-- 00cb:50           push ax
ffae 0010 ffe0 0004 ffac ffc6 0000 0000 -S-- 00cc:b80400       mov ax, 0004
0004 0010 ffe0 0004 ffac ffc6 0000 0000 -S-- 00cf:50           push ax
0004 0010 ffe0 0004 ffaa ffc6 0000 0000 -S-- 00d0:b80100       mov ax, 0001
0001 0010 ffe0 0004 ffaa ffc6 0000 0000 -S-- 00d3:50           push ax
0001 0010 ffe0 0004 ffa8 ffc6 0000 0000 -S-- 00d4:e80600       call 00dd
0001 0010 ffe0 0004 ffa6 ffc6 0000 0000 -S-- 00dd:55           push bp
0001 0010 ffe0 0004 ffa4 ffc6 0000 0000 -S-- 00de:89e5         mov bp, sp
0001 0010 ffe0 0004 ffa4 ffa4 0000 0000 -S-- 00e0:56           push si
0001 0010 ffe0 0004 ffa2 ffa4 0000 0000 -S-- 00e1:57           push di
0001 0010 ffe0 0004 ffa0 ffa4 0000 0000 -S-- 00e2:8b7608       mov si, [bp+8] ;[ffac]ffae
0001 0010 ffe0 0004 ffa0 ffa4 ffae 0000 -S-- 00e5:8b5606       mov dx, [bp+6] ;[ffaa]0004
0001 0010 ffe0 0004 ffa0 ffa4 ffae 0000 -S-- 00e8:895402       mov [si+2], dx ;[ffb0]0000
0001 0010 ffe0 0004 ffa0 ffa4 ffae 0000 -S-- 00eb:56           push si
0001 0010 ffe0 0004 ff9e ffa4 ffae 0000 -S-- 00ec:ff7604       push [bp+4] ;[ffa8]0001
0001 0010 ffe0 0004 ff9c ffa4 ffae 0000 -S-- 00ef:e82f00       call 0121
0001 0010 ffe0 0004 ff9a ffa4 ffae 0000 -S-- 0121:b90300       mov cx, 0003
0001 0010 0003 0004 ff9a ffa4 ffae 0000 -S-- 0124:eb00         jmp short 0126
0001 0010 0003 0004 ff9a ffa4 ffae 0000 -S-- 0126:55           push bp
0001 0010 0003 0004 ff98 ffa4 ffae 0000 -S-- 0127:89e5         mov bp, sp
0001 0010 0003 0004 ff98 ff98 ffae 0000 -S-- 0129:8b4604       mov ax, [bp+4] ;[ff9c]0001
0001 0010 0003 0004 ff98 ff98 ffae 0000 -S-- 012c:8b5e06       mov bx, [bp+6] ;[ff9e]ffae
0001 ffae 0003 0004 ff98 ff98 ffae 0000 -S-- 012f:cd20         int 20
<write(1, 0x0004, 6)hello
 => 6>
0000 ffae 0003 0004 ff98 ff98 ffae 0000 -S-- 0131:5d           pop bp
0000 ffae 0003 0004 ff9a ffa4 ffae 0000 -S-- 0132:c3           ret
0000 ffae 0003 0004 ff9c ffa4 ffae 0000 -S-- 00f2:5b           pop bx
0000 0001 0003 0004 ff9e ffa4 ffae 0000 -S-- 00f3:5b           pop bx
0000 ffae 0003 0004 ffa0 ffa4 ffae 0000 -S-- 00f4:89c7         mov di, ax
0000 ffae 0003 0004 ffa0 ffa4 ffae 0000 -S-- 00f6:09ff         or di, di
0000 ffae 0003 0004 ffa0 ffa4 ffae 0000 --Z- 00f8:7403         je 00fd
0000 ffae 0003 0004 ffa0 ffa4 ffae 0000 --Z- 00fd:837c0200     cmp [si+2], 0 ;[ffb0]0006
0000 ffae 0003 0004 ffa0 ffa4 ffae 0000 ---- 0101:7d0e         jnl 0111
0000 ffae 0003 0004 ffa0 ffa4 ffae 0000 ---- 0111:8b4402       mov ax, [si+2] ;[ffb0]0006
0006 ffae 0003 0004 ffa0 ffa4 ffae 0000 ---- 0114:e91c00       jmp 0133
0006 ffae 0003 0004 ffa0 ffa4 ffae 0000 ---- 0133:5f           pop di
0006 ffae 0003 0004 ffa2 ffa4 ffae 0000 ---- 0134:5e           pop si
0006 ffae 0003 0004 ffa4 ffa4 0000 0000 ---- 0135:89ec         mov sp, bp
0006 ffae 0003 0004 ffa4 ffa4 0000 0000 ---- 0137:5d           pop bp
0006 ffae 0003 0004 ffa6 ffc6 0000 0000 ---- 0138:c3           ret
0006 ffae 0003 0004 ffa8 ffc6 0000 0000 ---- 00d7:83c406       add sp, 6
0006 ffae 0003 0004 ffae ffc6 0000 0000 -S-- 00da:e95800       jmp 0135
0006 ffae 0003 0004 ffae ffc6 0000 0000 -S-- 0135:89ec         mov sp, bp
0006 ffae 0003 0004 ffc6 ffc6 0000 0000 -S-- 0137:5d           pop bp
0006 ffae 0003 0004 ffc8 ffd0 0000 0000 -S-- 0138:c3           ret
0006 ffae 0003 0004 ffca ffd0 0000 0000 -S-- 004b:83c406       add sp, 6
0006 ffae 0003 0004 ffd0 ffd0 0000 0000 -S-- 004e:e9e400       jmp 0135
0006 ffae 0003 0004 ffd0 ffd0 0000 0000 -S-- 0135:89ec         mov sp, bp
0006 ffae 0003 0004 ffd0 ffd0 0000 0000 -S-- 0137:5d           pop bp
0006 ffae 0003 0004 ffd2 0000 0000 0000 -S-- 0138:c3           ret
0006 ffae 0003 0004 ffd4 0000 0000 0000 -S-- 0034:50           push ax
0006 ffae 0003 0004 ffd2 0000 0000 0000 -S-- 0035:e83300       call 006b
0006 ffae 0003 0004 ffd0 0000 0000 0000 -S-- 006b:55           push bp
0006 ffae 0003 0004 ffce 0000 0000 0000 -S-- 006c:89e5         mov bp, sp
0006 ffae 0003 0004 ffce ffce 0000 0000 -S-- 006e:e8e0ff       call 0051
0006 ffae 0003 0004 ffcc ffce 0000 0000 -S-- 0051:55           push bp
0006 ffae 0003 0004 ffca ffce 0000 0000 -S-- 0052:89e5         mov bp, sp
0006 ffae 0003 0004 ffca ffca 0000 0000 -S-- 0054:56           push si
0006 ffae 0003 0004 ffc8 ffca 0000 0000 -S-- 0055:8b360c00     mov si, [000c] ;[000c]0000
0006 ffae 0003 0004 ffc8 ffca 0000 0000 -S-- 0059:4e           dec si
0006 ffae 0003 0004 ffc8 ffca ffff 0000 -S-- 005a:7c0c         jl 0068
0006 ffae 0003 0004 ffc8 ffca ffff 0000 -S-- 0068:e9c900       jmp 0134
0006 ffae 0003 0004 ffc8 ffca ffff 0000 -S-- 0134:5e           pop si
0006 ffae 0003 0004 ffca ffca 0000 0000 -S-- 0135:89ec         mov sp, bp
0006 ffae 0003 0004 ffca ffca 0000 0000 -S-- 0137:5d           pop bp
0006 ffae 0003 0004 ffcc ffce 0000 0000 -S-- 0138:c3           ret
0006 ffae 0003 0004 ffce ffce 0000 0000 -S-- 0071:833e0e0000   cmp [000e], 0 ;[000e]0000
0006 ffae 0003 0004 ffce ffce 0000 0000 --Z- 0076:7406         je 007e
0006 ffae 0003 0004 ffce ffce 0000 0000 --Z- 007e:ff7604       push [bp+4] ;[ffd2]0006
0006 ffae 0003 0004 ffcc ffce 0000 0000 --Z- 0081:e80400       call 0088
0006 ffae 0003 0004 ffca ffce 0000 0000 --Z- 0088:e90500       jmp 0090
0006 ffae 0003 0004 ffca ffce 0000 0000 --Z- 0090:55           push bp
0006 ffae 0003 0004 ffc8 ffce 0000 0000 --Z- 0091:89e5         mov bp, sp
0006 ffae 0003 0004 ffc8 ffc8 0000 0000 --Z- 0093:83ec18       sub sp, 18
0006 ffae 0003 0004 ffb0 ffc8 0000 0000 -S-- 0096:8b5604       mov dx, [bp+4] ;[ffcc]0006
0006 ffae 0003 0006 ffb0 ffc8 0000 0000 -S-- 0099:8956ec       mov [bp-14], dx ;[ffb4]0006
0006 ffae 0003 0006 ffb0 ffc8 0000 0000 -S-- 009c:8d46e8       lea ax, [bp-18] ;[ffb0]0006
ffb0 ffae 0003 0006 ffb0 ffc8 0000 0000 -S-- 009f:50           push ax
ffb0 ffae 0003 0006 ffae ffc8 0000 0000 -S-- 00a0:b80100       mov ax, 0001
0001 ffae 0003 0006 ffae ffc8 0000 0000 -S-- 00a3:50           push ax
0001 ffae 0003 0006 ffac ffc8 0000 0000 -S-- 00a4:31c0         xor ax, ax
0000 ffae 0003 0006 ffac ffc8 0000 0000 --Z- 00a6:50           push ax
0000 ffae 0003 0006 ffaa ffc8 0000 0000 --Z- 00a7:e83300       call 00dd
0000 ffae 0003 0006 ffa8 ffc8 0000 0000 --Z- 00dd:55           push bp
0000 ffae 0003 0006 ffa6 ffc8 0000 0000 --Z- 00de:89e5         mov bp, sp
0000 ffae 0003 0006 ffa6 ffa6 0000 0000 --Z- 00e0:56           push si
0000 ffae 0003 0006 ffa4 ffa6 0000 0000 --Z- 00e1:57           push di
0000 ffae 0003 0006 ffa2 ffa6 0000 0000 --Z- 00e2:8b7608       mov si, [bp+8] ;[ffae]ffb0
0000 ffae 0003 0006 ffa2 ffa6 ffb0 0000 --Z- 00e5:8b5606       mov dx, [bp+6] ;[ffac]0001
0000 ffae 0003 0001 ffa2 ffa6 ffb0 0000 --Z- 00e8:895402       mov [si+2], dx ;[ffb2]0001
0000 ffae 0003 0001 ffa2 ffa6 ffb0 0000 --Z- 00eb:56           push si
0000 ffae 0003 0001 ffa0 ffa6 ffb0 0000 --Z- 00ec:ff7604       push [bp+4] ;[ffaa]0000
0000 ffae 0003 0001 ff9e ffa6 ffb0 0000 --Z- 00ef:e82f00       call 0121
0000 ffae 0003 0001 ff9c ffa6 ffb0 0000 --Z- 0121:b90300       mov cx, 0003
0000 ffae 0003 0001 ff9c ffa6 ffb0 0000 --Z- 0124:eb00         jmp short 0126
0000 ffae 0003 0001 ff9c ffa6 ffb0 0000 --Z- 0126:55           push bp
0000 ffae 0003 0001 ff9a ffa6 ffb0 0000 --Z- 0127:89e5         mov bp, sp
0000 ffae 0003 0001 ff9a ff9a ffb0 0000 --Z- 0129:8b4604       mov ax, [bp+4] ;[ff9e]0000
0000 ffae 0003 0001 ff9a ff9a ffb0 0000 --Z- 012c:8b5e06       mov bx, [bp+6] ;[ffa0]ffb0
0000 ffb0 0003 0001 ff9a ff9a ffb0 0000 --Z- 012f:cd20         int 20
<exit(6)>
````
