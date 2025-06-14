pub struct Interpreter<'a> {
    it: std::str::Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("예상치 못한 기호 '{op}'");
            }
        }
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("예상치 못한 기호 '{ch}'"),
            None => panic!("예상치 못한 문자열의 끝"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter() {
        let mut intr = Interpreter::new("2+3");
        let mut postfix = String::new();
        intr.interpret(&mut postfix);
        assert_eq!(postfix, "23+");

        intr = Interpreter::new("1-2+3-4");
        postfix.clear();
        intr.interpret(&mut postfix);
        assert_eq!(postfix, "12-3+4-");
    }
}
