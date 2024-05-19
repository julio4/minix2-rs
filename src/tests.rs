use pretty_assertions::assert_eq;

use crate::disassembler::minix2_disassemble;

#[test]
fn test_asem_1() {
    let args = vec![
        "minix2_rs".to_string(),
        "./tests_data/asem/1.s.out".to_string(),
    ];
    let result = minix2_disassemble(args).unwrap();
    let expected = "0000: bb0000        mov bx, 0000
0003: cd20          int 20
0005: bb1000        mov bx, 0010
0008: cd20          int 20
000a: 0000          add [bx+si], al
000c: 0000          add [bx+si], al
000e: 0000          add [bx+si], al
";
    assert_eq!(result, expected);
}

#[test]
fn test_asem_2() {
    let args = vec![
        "minix2_rs".to_string(),
        "./tests_data/asem/2.s.out".to_string(),
    ];
    let result = minix2_disassemble(args).unwrap();
    let expected = "0000: 832e210020    sub [0021], 20
0005: bb0000        mov bx, 0000
0008: cd20          int 20
000a: bb1000        mov bx, 0010
000d: cd20          int 20
000f: 00            (undefined)
";
    assert_eq!(result, expected);
}

#[test]
fn test_asem_3() {
    let args = vec![
        "minix2_rs".to_string(),
        "./tests_data/asem/3.s.out".to_string(),
    ];
    let result = minix2_disassemble(args).unwrap();
    let expected = "0000: 832e210020    sub [0021], 20
0005: b90000        mov cx, 0000
0008: 89cb          mov bx, cx
000a: cd20          int 20
000c: bb1000        mov bx, 0010
000f: cd20          int 20
0011: 0000          add [bx+si], al
0013: 0000          add [bx+si], al
0015: 0000          add [bx+si], al
0017: 0000          add [bx+si], al
0019: 0000          add [bx+si], al
001b: 0000          add [bx+si], al
001d: 0000          add [bx+si], al
001f: 00            (undefined)
";
    assert_eq!(result, expected);
}

#[test]
fn test_asem_4() {
    let args = vec![
        "minix2_rs".to_string(),
        "./tests_data/asem/4.s.out".to_string(),
    ];
    let result = minix2_disassemble(args).unwrap();
    let expected = "0000: 832e210020    sub [0021], 20
0005: b90000        mov cx, 0000
0008: 89c8          mov ax, cx
000a: 89c3          mov bx, ax
000c: cd20          int 20
000e: bb1000        mov bx, 0010
0011: cd20          int 20
0013: 0000          add [bx+si], al
0015: 0000          add [bx+si], al
0017: 0000          add [bx+si], al
0019: 0000          add [bx+si], al
001b: 0000          add [bx+si], al
001d: 0000          add [bx+si], al
001f: 00            (undefined)
";
    assert_eq!(result, expected);
}

#[test]
fn test_c_1() {
    let args = vec!["minix2_rs".to_string(), "./tests_data/1.c.out".to_string()];
    let result = minix2_disassemble(args).unwrap();
    let expected = "0000: 31ed          xor bp, bp
0002: 89e3          mov bx, sp
0004: 8b07          mov ax, [bx]
0006: 8d5702        lea dx, [bx+2]
0009: 8d4f04        lea cx, [bx+4]
000c: 01c1          add cx, ax
000e: 01c1          add cx, ax
0010: bb1000        mov bx, 0010
0013: 81fb1400      cmp bx, 0014
0017: 730f          jnb 0028
0019: f6c301        test bl, 1
001c: 750a          jne 0028
001e: 813f5353      cmp [bx], 5353
0022: 7504          jne 0028
0024: 891e0200      mov [0002], bx
0028: 8b1e0200      mov bx, [0002]
002c: 890f          mov [bx], cx
002e: 51            push cx
002f: 52            push dx
0030: 50            push ax
0031: e80500        call 0039
0034: 50            push ax
0035: e83300        call 006b
0038: f4            hlt
0039: 55            push bp
003a: 89e5          mov bp, sp
003c: b80600        mov ax, 0006
003f: 50            push ax
0040: b80400        mov ax, 0004
0043: 50            push ax
0044: b80100        mov ax, 0001
0047: 50            push ax
0048: e84100        call 008c
004b: 83c406        add sp, 6
004e: e9e400        jmp 0135
0051: 55            push bp
0052: 89e5          mov bp, sp
0054: 56            push si
0055: 8b360c00      mov si, [000c]
0059: 4e            dec si
005a: 7c0c          jl 0068
005c: 89f3          mov bx, si
005e: d1e3          shl bx, 1
0060: 8b9f1600      mov bx, [bx+16]
0064: ffd3          call bx
0066: ebf1          jmp short 0059
0068: e9c900        jmp 0134
006b: 55            push bp
006c: 89e5          mov bp, sp
006e: e8e0ff        call 0051
0071: 833e0e0000    cmp [000e], 0
0076: 7406          je 007e
0078: 8b1e0e00      mov bx, [000e]
007c: ffd3          call bx
007e: ff7604        push [bp+4]
0081: e80400        call 0088
0084: 5b            pop bx
0085: e9ad00        jmp 0135
0088: e90500        jmp 0090
008b: 00e9          add cl, ch
008d: 2100          and [bx+si], ax
008f: 005589        add [di-77], dl
0092: e583          in ax, 83
0094: ec            in al, dx
0095: 188b5604      sbb [bp+di+456], cl
0099: 8956ec        mov [bp-14], dx
009c: 8d46e8        lea ax, [bp-18]
009f: 50            push ax
00a0: b80100        mov ax, 0001
00a3: 50            push ax
00a4: 31c0          xor ax, ax
00a6: 50            push ax
00a7: e83300        call 00dd
00aa: 83c406        add sp, 6
00ad: e98500        jmp 0135
00b0: 55            push bp
00b1: 89e5          mov bp, sp
00b3: 83ec18        sub sp, 18
00b6: 8b5604        mov dx, [bp+4]
00b9: 8956ec        mov [bp-14], dx
00bc: 8b5608        mov dx, [bp+8]
00bf: 8956ee        mov [bp-12], dx
00c2: 8b5606        mov dx, [bp+6]
00c5: 8956f2        mov [bp-e], dx
00c8: 8d46e8        lea ax, [bp-18]
00cb: 50            push ax
00cc: b80400        mov ax, 0004
00cf: 50            push ax
00d0: b80100        mov ax, 0001
00d3: 50            push ax
00d4: e80600        call 00dd
00d7: 83c406        add sp, 6
00da: e95800        jmp 0135
00dd: 55            push bp
00de: 89e5          mov bp, sp
00e0: 56            push si
00e1: 57            push di
00e2: 8b7608        mov si, [bp+8]
00e5: 8b5606        mov dx, [bp+6]
00e8: 895402        mov [si+2], dx
00eb: 56            push si
00ec: ff7604        push [bp+4]
00ef: e82f00        call 0121
00f2: 5b            pop bx
00f3: 5b            pop bx
00f4: 89c7          mov di, ax
00f6: 09ff          or di, di
00f8: 7403          je 00fd
00fa: 897c02        mov [si+2], di
00fd: 837c0200      cmp [si+2], 0
0101: 7d0e          jnl 0111
0103: 8b5402        mov dx, [si+2]
0106: f7da          neg dx
0108: 89161200      mov [0012], dx
010c: b8ffff        mov ax, ffff
010f: eb03          jmp short 0114
0111: 8b4402        mov ax, [si+2]
0114: e91c00        jmp 0133
0117: b90100        mov cx, 0001
011a: eb0a          jmp short 0126
011c: b90200        mov cx, 0002
011f: eb05          jmp short 0126
0121: b90300        mov cx, 0003
0124: eb00          jmp short 0126
0126: 55            push bp
0127: 89e5          mov bp, sp
0129: 8b4604        mov ax, [bp+4]
012c: 8b5e06        mov bx, [bp+6]
012f: cd20          int 20
0131: 5d            pop bp
0132: c3            ret
0133: 5f            pop di
0134: 5e            pop si
0135: 89ec          mov sp, bp
0137: 5d            pop bp
0138: c3            ret
0139: 5e            pop si
013a: 5f            pop di
013b: ebf8          jmp short 0135
013d: 0000          add [bx+si], al
013f: 00            (undefined)
";
    assert_eq!(result, expected);
}

