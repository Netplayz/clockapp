pub fn clean_bf(program: &[u8]) -> Vec<u8> {
    program.iter().filter(|&&b| matches!(b, b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']')).copied().collect()
}

pub fn interpret(program: &[u8], input: &[u8]) -> Vec<u8> {
    const TAPE_SIZE: usize = 30_000;
    let mut tape = vec![0u8; TAPE_SIZE];
    let mut dp: usize = 0;
    let mut ip: usize = 0;
    let mut input_pos: usize = 0;
    let mut output = Vec::new();

    let mut jump = vec![0usize; program.len()];
    let mut stack = Vec::new();
    for (i, &b) in program.iter().enumerate() {
        match b {
            b'[' => stack.push(i),
            b']' => {
                if let Some(open) = stack.pop() {
                    jump[open] = i;
                    jump[i] = open;
                }
            }
            _ => {}
        }
    }

    while ip < program.len() {
        match program[ip] {
            b'>' => dp = (dp + 1) % TAPE_SIZE,
            b'<' => dp = (dp + TAPE_SIZE - 1) % TAPE_SIZE,
            b'+' => tape[dp] = tape[dp].wrapping_add(1),
            b'-' => tape[dp] = tape[dp].wrapping_sub(1),
            b'.' => output.push(tape[dp]),
            b',' => {
                tape[dp] = if input_pos < input.len() {
                    let val = input[input_pos];
                    input_pos += 1;
                    val
                } else {
                    0
                };
            }
            b'[' => {
                if tape[dp] == 0 {
                    ip = jump[ip];
                }
            }
            b']' => {
                if tape[dp] != 0 {
                    ip = jump[ip];
                }
            }
            _ => {}
        }
        ip += 1;
    }

    output
}

pub fn interpret_str(program: &str, input: &[u8]) -> Vec<u8> {
    interpret(program.as_bytes(), input)
}

pub fn run_bf(program: &str, input: &[u8]) -> Result<String, String> {
    let cleaned = clean_bf(program.as_bytes());
    let output = interpret(&cleaned, input);
    String::from_utf8(output).map_err(|e| format!("output is not valid UTF-8: {}", e))
}

pub const TICKTOCK_BF: &str = include_str!("../../bf/ticktock.bf");
pub const BINARY_CLOCK_BF: &str = include_str!("../../bf/binary_clock.bf");
pub const PRIME_MINUTE_BF: &str = include_str!("../../bf/prime_minute.bf");
pub const ASCII_DIGITS_BF: &str = include_str!("../../bf/ascii_digits.bf");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_output() {
        assert_eq!(interpret(b"+++.", b""), b"\x03");
    }

    #[test]
    fn test_cat_two_bytes() {
        assert_eq!(interpret(b",.,.", b"ab"), b"ab");
    }

    #[test]
    fn test_eof_sets_zero() {
        assert_eq!(interpret(b",.", b""), b"\x00");
    }

    #[test]
    fn test_hello_world() {
        let program = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let output = interpret(program, b"");
        assert_eq!(output, b"Hello World!\n");
    }

    #[test]
    fn test_nested_loops() {
        let output = interpret(b"++[>++[>+++<-]<-]>>.", b"");
        assert_eq!(output, b"\x0c");
    }

    #[test]
    fn test_interpret_str() {
        assert_eq!(interpret_str("+++.", b""), b"\x03");
    }

    #[test]
    fn test_run_bf_valid_utf8() {
        let result = run_bf("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", b"");
        assert_eq!(result.unwrap(), "Hello World!\n");
    }

    #[test]
    fn test_run_bf_invalid_utf8() {
        let result = run_bf("-.", b"");
        assert!(result.is_err());
    }

    #[test]
    fn test_wrapping_data_pointer() {
        let mut prog = Vec::new();
        for _ in 0..30_000 {
            prog.push(b'>');
        }
        prog.push(b'+');
        prog.push(b'.');
        let output = interpret(&prog, b"");
        assert_eq!(output, b"\x01");
    }

    #[test]
    fn test_wrapping_cell() {
        assert_eq!(interpret(b"-." , b""), b"\xff");
        assert_eq!(interpret(b"+." , b""), b"\x01");
        assert_eq!(interpret(b"+++++ +++++ +++++ +++++ +++++ +++++ .", b""), b"\x1e");
    }

    #[test]
    fn test_skip_loop_when_zero() {
        assert_eq!(interpret(b"[+++].", b""), b"\x00");
    }

    #[test]
    fn test_wrapping_decrement_pointer() {
        let mut prog = vec![b'<'; 1];
        prog.push(b'+');
        prog.push(b'.');
        let output = interpret(&prog, b"");
        assert_eq!(output, b"\x01");
    }

    #[test]
    fn test_comments_ignored() {
        assert_eq!(interpret(b"+++ blah .", b""), b"\x03");
    }

    // ---- BF program integration tests ----

    fn run_program(name: &str, input: &[u8]) -> Vec<u8> {
        let code = match name {
            "ticktock.bf" => include_str!("../../bf/ticktock.bf"),
            "binary_clock.bf" => include_str!("../../bf/binary_clock.bf"),
            "ascii_digits.bf" => include_str!("../../bf/ascii_digits.bf"),
            "prime_minute.bf" => include_str!("../../bf/prime_minute.bf"),
            _ => panic!("unknown BF program: {}", name),
        };
        interpret(&clean_bf(code.as_bytes()), input)
    }

    #[test]
    fn test_ticktock_once() {
        let out = run_program("ticktock.bf", b"1");
        assert_eq!(out, b"tick\ntock\n");
    }

    #[test]
    fn test_ticktock_thrice() {
        let out = run_program("ticktock.bf", b"3");
        assert_eq!(out, b"tick\ntock\ntick\ntock\ntick\ntock\n");
    }

    #[test]
    fn test_ticktock_zero() {
        let out = run_program("ticktock.bf", b"0");
        assert_eq!(out, b"");
    }

    #[test]
    fn test_binary_clock() {
        let out = run_program("binary_clock.bf", b"\x03\x05\x02");
        assert_eq!(out, b"...\n.....\n..\n");
    }

    #[test]
    fn test_binary_clock_zero() {
        let out = run_program("binary_clock.bf", b"\x00\x00\x00");
        assert_eq!(out, b"\n\n\n");
    }

    #[test]
    fn test_ascii_digits_five() {
        let out = run_program("ascii_digits.bf", b"5");
        assert_eq!(out, b"@@@@@\n");
    }

    #[test]
    fn test_ascii_digits_zero() {
        let out = run_program("ascii_digits.bf", b"0");
        assert_eq!(out, b"\n");
    }

    #[test]
    fn test_prime_minute_2() {
        let out = run_program("prime_minute.bf", b"\x02");
        assert_eq!(out, b"Y");
    }

    #[test]
    fn test_prime_minute_4() {
        let out = run_program("prime_minute.bf", b"\x04");
        assert_eq!(out, b"N");
    }

    #[test]
    fn test_prime_minute_0() {
        let out = run_program("prime_minute.bf", b"\x00");
        assert_eq!(out, b"N");
    }

    #[test]
    fn test_prime_minute_1() {
        let out = run_program("prime_minute.bf", b"\x01");
        assert_eq!(out, b"N");
    }

    #[test]
    fn test_prime_minute_59() {
        let out = run_program("prime_minute.bf", b"\x3B");
        assert_eq!(out, b"Y");
    }

    #[test]
    fn test_prime_minute_53() {
        let out = run_program("prime_minute.bf", b"\x35");
        assert_eq!(out, b"Y");
    }

    #[test]
    fn test_prime_minute_all_primes() {
        let primes: [u8; 17] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59];
        for p in primes {
            let out = run_program("prime_minute.bf", &[p]);
            assert_eq!(out, b"Y", "expected Y for prime {}", p);
        }
    }

    #[test]
    fn test_prime_minute_all_nonprimes() {
        for n in 0u8..=59 {
            let is_prime = matches!(n, 2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 | 23 | 29 | 31 | 37 | 41 | 43 | 47 | 53 | 59);
            assert_eq!(
                run_program("prime_minute.bf", &[n]),
                if is_prime { b"Y" } else { b"N" },
                "mismatch for n={}", n
            );
        }
    }
}
