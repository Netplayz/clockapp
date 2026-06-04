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
    let output = interpret(program.as_bytes(), input);
    String::from_utf8(output).map_err(|e| e.to_string())
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
}
