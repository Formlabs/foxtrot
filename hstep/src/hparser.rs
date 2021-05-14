use std::fs::File;
use std::io;
// use std::io::BufRead;
use std::io::Read;
// use std::str;
// use std::collections::HashMap;
// use std::path::Path;

pub struct Id(pub usize);

fn read_char_from_stream(stream: &mut io::BufReader<File>) -> Option<char> {
    let mut buf = vec![0u8; 1];
    let byte_or_error = stream.read_exact(&mut buf);
    let result = match byte_or_error {
        Ok(()) => Some(buf[0] as char),
        Err(err) => match err.kind() {
            io::ErrorKind::UnexpectedEof => None,
            _ => panic!("bad error kind in read_char"),
        },
    };
    result
}

pub struct PeekableBufReader {
    stream: io::BufReader<File>,
    next_char: Option<char>,
}

impl PeekableBufReader {
    pub fn new(stream: io::BufReader<File>) -> PeekableBufReader {
        let mut result = PeekableBufReader {
            stream: stream,
            next_char: None,
        };
        result.next_char = read_char_from_stream(&mut result.stream);
        result
    }

    fn peek_char(&mut self) -> char {
        self.next_char.unwrap()
    }
    fn has_char(&mut self) -> bool {
        self.next_char.is_some()
    }
    fn read_char(&mut self) -> char {
        let result = self.next_char.unwrap();
        self.next_char = read_char_from_stream(&mut self.stream);
        // print!("{}", result);

        result
    }
    // When this finishes, either this functionw ill panic, or peek_char will be delim
    fn read_up_to(&mut self, delim: char) -> String {
        let mut cs = "".to_string();
        loop {
            if !self.has_char() {
                panic!("Reached end of file without finding delim");
            }
            let c = self.peek_char();
            if c == delim {
                return cs;
            }
            cs.push(self.read_char());
        }
    }
}

pub struct Parser {
    pub stream: PeekableBufReader,
}

impl Parser {
    fn skip_comment(&mut self) {
        let c = self.stream.read_char();
        if c != '/' {
            panic!("expected / at start of comment");
        }
        let c = self.stream.read_char();
        if c != '*' {
            panic!("/ without *");
        }
        loop {
            let c = self.stream.read_char();
            if c == '*' && self.stream.peek_char() == '/' {
                self.stream.read_char();
                break;
            }
        }
    }
    pub fn skip_whitespace(&mut self) {
        loop {
            if !self.stream.has_char() {
                break;
            }
            let c = self.stream.peek_char();
            match c {
                ' ' | '\n' | '\r' | '\t' => {
                    self.stream.read_char();
                }
                '/' => {
                    self.skip_comment();
                }
                _ => break,
            }
        }
    }
    pub fn skip_expect_whitespace(&mut self) {
        if !self.stream.has_char() {
            panic!("expected whitespace, not eof")
        }
        let c = self.stream.peek_char();
        match c {
            ' ' | '\n' | '\r' | '\t' => {
                self.stream.read_char();
            }
            _ => panic!("expected whitespace, not >>{}<<", c),
        }
        self.skip_whitespace();
    }
    pub fn read_identifier(&mut self) -> String {
        if !self.stream.has_char() {
            panic!("expected character for identifier, not eof")
        }
        let mut cs = "".to_string();
        cs.push(self.stream.peek_char());
        self.stream.read_char();
        loop {
            if !self.stream.has_char() {
                break;
            }
            let c = self.stream.peek_char();
            match c {
                'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => {
                    cs.push(c);
                },
                ' ' | '\n' | '\r' | '\t' | '(' | ')' | '=' | ';' | ',' => break,
                _ => panic!("unexpected character >>{}<<", c),
            }
            self.stream.read_char();
        }
        return cs;
    }
    pub fn read_id(&mut self) -> Id {
        if !self.stream.has_char() {
            panic!("expected character for identifier, not eof")
        }
        if self.stream.peek_char() != '#' {
            panic!("expected index to start with #")
        }
        self.stream.read_char();
        let mut cs = "".to_string();
        loop {
            if !self.stream.has_char() {
                break;
            }
            let c = self.stream.peek_char();
            match c {
                ' ' | '\n' | '\r' | '\t' | '(' | ')' | '=' | ';' | ',' => break,
                '0'..='9' => {
                    cs.push(c);
                }
                _ => panic!("unexpected character"),
            }
            self.stream.read_char();
        }
        return Id(cs.parse::<usize>().unwrap());
    }
    pub fn read_literal(&mut self) -> String {
        if !self.stream.has_char() {
            panic!("expected character for identifier, not eof")
        }
        if self.stream.peek_char() != '.' {
            panic!("expected unit to start with .")
        }
        self.stream.read_char();
        let mut cs = "".to_string();
        loop {
            if !self.stream.has_char() {
                panic!("expected closing '.'")
            }
            let c = self.stream.peek_char();
            match c {
                '.' => {
                    self.stream.read_char();
                    break;
                }
                'A'..='Z' | 'a'..='z' | '_' => {
                    cs.push(c);
                }
                _ => panic!("unexpected character >>{}<<", c),
            }
            self.stream.read_char();
        }
        return cs;
    }
    pub fn read_bool(&mut self) -> bool {
        self.read_period();
        let c = self.stream.read_char();
        let v = match c {
            'T' => true,
            'F' => false,
            _ => {
                panic!("unexpected char >>{}<<", c);
            }
        };
        self.read_period();
        v
    }
    pub fn read_float(&mut self) -> f64 {
        if !self.stream.has_char() {
            panic!("expected character for number, not eof")
        }
        let c = self.stream.read_char();
        let mut cs = "".to_string();
        cs.push(c);
        loop {
            let c = self.stream.peek_char();
            match c {
                '-' | '0'..='9' => {
                    cs.push(c);
                }
                '.' => {
                    cs.push(c);
                    self.stream.read_char();
                    break;
                }
                _ => panic!("unexpected character >>{}<<", c),
            }
            self.stream.read_char();
        }
        // now we are parsing a float. First read the part after the decimal place
        loop {
            if !self.stream.has_char() {
                return cs.parse::<f64>().unwrap();
            }
            let c = self.stream.peek_char();
            match c {
                ' ' | '\n' | '\r' | '\t' | '(' | ')' | '=' | ';' | ',' => {
                    return cs.parse::<f64>().unwrap();
                }
                '0'..='9' => {
                    cs.push(c);
                }
                'E' => {
                    self.stream.read_char();
                    break;
                }
                _ => panic!("unexpected character >>{}<<", c),
            }
            self.stream.read_char();
        }
        // now we are parsing the exponent
        let mut exp_cs = "".to_string();
        // parse the + or -
        if !self.stream.has_char() {
            panic!("should have + or -");
        }
        let c = self.stream.peek_char();
        match c {
            '+' | '-' => {
                exp_cs.push(c);
            }
            _ => panic!("should have + or -"),
        }
        self.stream.read_char();

        // parse the first num character
        if !self.stream.has_char() {
            panic!("should have digit");
        }
        let c = self.stream.peek_char();
        match c {
            '0'..='9' => {
                exp_cs.push(c);
            }
            _ => panic!("should have digit"),
        }
        self.stream.read_char();

        // the parse the int
        loop {
            if !self.stream.has_char() {
                break;
            }
            let c = self.stream.peek_char();
            match c {
                ' ' | '\n' | '\r' | '\t' | '(' | ')' | '=' | ';' | ',' => break,
                '0'..='9' => {
                    exp_cs.push(c);
                }
                _ => panic!("unexpected character >>{}<<", c),
            }
            self.stream.read_char();
        }
        return cs.parse::<f64>().unwrap() * ((10.0f64).powi(exp_cs.parse::<i32>().unwrap()));
    }

    pub fn read_int(&mut self) -> i32 {
        let c = self.stream.read_char();
        let mut cs = "".to_string();

        match c {
            '-' | '0'..='9' => {
                cs.push(c);
            }
            _ => panic!("unexpected character >>{}<<", c),
        }
        loop {
            if !self.stream.has_char() {
                break;
            }
            let c = self.stream.peek_char();
            match c {
                ' ' | '\n' | '\r' | '\t' | '(' | ')' | '=' | ';' | ',' => {
                    break;
                }
                '0'..='9' => {
                    cs.push(c);
                }
                _ => panic!("unexpected character >>{}<<", c),
            }
            self.stream.read_char();
        }
        return cs.parse::<i32>().unwrap();
    }

    pub fn read_string(&mut self) -> String {
        self.read_quote();
        let v = self.stream.read_up_to('\'');
        self.read_quote();
        v
    }

    pub fn read_united_float(&mut self) -> (String, f64) {
        let iden = self.read_identifier();
        self.skip_whitespace();
        self.read_open_paren();
        self.skip_whitespace();
        let val = self.read_float();
        self.skip_whitespace();
        self.read_close_paren();
        self.skip_whitespace();
        (iden, val)
    }

    pub fn read_assert_char(&mut self, expected: char) {
        let c = self.stream.read_char();
        if c != expected {
            panic!("expected {} but got >>{}<<", expected, c);
        }
    }

    pub fn read_vector<T, F>(&mut self, func: F) -> Vec<T>
    where
        F: Fn(&mut Parser) -> T,
    {
        let mut result: Vec<T> = Vec::new();
        self.read_open_paren();
        self.skip_whitespace();
        if self.peek_char() == ')' {
            self.read_close_paren();
            return result;
        }
        let i = func(self);
        self.skip_whitespace();
        result.push(i);
        loop {
            if self.peek_char() == ')' {
                self.read_close_paren();
                break;
            }
            self.read_comma();
            self.skip_whitespace();
            let i = func(self);
            self.skip_whitespace();
            result.push(i);
        }
        result
    }

    pub fn read_or_dollar<T, F>(&mut self, func: F) -> Option<T>
    where
        F: Fn(&mut Parser) -> T,
    {
        match self.peek_char() {
            '$' => {
                self.read_dollar();
                None
            }
            _ => Some(func(self)),
        }
    }

    pub fn read_pair<T1, F1, T2, F2>(&mut self, func1: F1, func2: F2) -> (T1, T2)
    where
        F1: Fn(&mut Parser) -> T1,
        F2: Fn(&mut Parser) -> T2,
    {
        self.read_open_paren();
        self.skip_whitespace();
        let a = func1(self);
        self.skip_whitespace();
        self.read_comma();
        self.skip_whitespace();
        let b = func2(self);
        self.skip_whitespace();
        self.read_close_paren();
        (a, b)
    }

    pub fn read_eof(&mut self) {
        if self.stream.has_char() {
            panic!("expected eof but got >>{}<<", self.stream.peek_char());
        }
    }

    pub fn read_semicolon(&mut self) {
        self.read_assert_char(';')
    }
    pub fn read_open_paren(&mut self) {
        self.read_assert_char('(')
    }
    pub fn read_close_paren(&mut self) {
        self.read_assert_char(')')
    }
    pub fn read_comma(&mut self) {
        self.read_assert_char(',')
    }
    pub fn read_period(&mut self) {
        self.read_assert_char('.')
    }
    pub fn read_star(&mut self) {
        self.read_assert_char('*')
    }
    pub fn read_quote(&mut self) {
        self.read_assert_char('\'')
    }
    pub fn read_equal_sign(&mut self) {
        self.read_assert_char('=')
    }
    pub fn read_dollar(&mut self) {
        self.read_assert_char('$')
    }
    pub fn read_id_or_dollar(&mut self) -> Option<Id> {
        self.read_or_dollar(Parser::read_id)
    }
    pub fn read_literal_or_dollar(&mut self) -> Option<String> {
        self.read_or_dollar(Parser::read_literal)
    }
    pub fn read_string_or_dollar(&mut self) -> Option<String> {
        self.read_or_dollar(Parser::read_string)
    }
    pub fn read_id_united_float_pair(&mut self) -> (Id, (String, f64)) {
        self.read_pair(Parser::read_id, Parser::read_united_float)
    }

    pub fn read_up_to(&mut self, delim: char) -> String {
        self.stream.read_up_to(delim)
    }
    pub fn peek_char(&mut self) -> char {
        self.stream.peek_char()
    }

    pub fn read_string_vector(&mut self) -> Vec<String> {
        self.read_vector(Parser::read_string)
    }
    pub fn read_int_vector(&mut self) -> Vec<i32> {
        self.read_vector(Parser::read_int)
    }
    pub fn read_float_vector(&mut self) -> Vec<f64> {
        self.read_vector(Parser::read_float)
    }
    pub fn read_float_vector_vector(&mut self) -> Vec<Vec<f64>> {
        self.read_vector(Parser::read_float_vector)
    }
    pub fn read_id_vector(&mut self) -> Vec<Id> {
        self.read_vector(Parser::read_id)
    }
    pub fn read_id_vector_vector(&mut self) -> Vec<Vec<Id>> {
        self.read_vector(Parser::read_id_vector)
    }


    // fn read_tuple_0(&mut self) -> () {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     self.read_close_paren();
    // }
    // fn read_tuple_1<T1, F1>(&mut self, func1: F1) -> T1
    // where
    //     F1: Fn(&mut Parser) -> T1,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     a
    // }
    // fn read_tuple_2<T1, F1, T2, F2>(&mut self, func1: F1, func2: F2) -> (T1, T2)
    // where
    //     F1: Fn(&mut Parser) -> T1,
    //     F2: Fn(&mut Parser) -> T2,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let b = func2(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     (a, b)
    // }
    // fn read_tuple_3<T1, F1, T2, F2, T3, F3>(
    //     &mut self,
    //     func1: F1,
    //     func2: F2,
    //     func3: F3,
    // ) -> (T1, T2, T3)
    // where
    //     F1: Fn(&mut Parser) -> T1,
    //     F2: Fn(&mut Parser) -> T2,
    //     F3: Fn(&mut Parser) -> T3,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let b = func2(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let c = func3(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     (a, b, c)
    // }
    // fn read_tuple_4<T1, F1, T2, F2, T3, F3, T4, F4>(
    //     &mut self,
    //     func1: F1,
    //     func2: F2,
    //     func3: F3,
    //     func4: F4,
    // ) -> (T1, T2, T3, T4)
    // where
    //     F1: Fn(&mut Parser) -> T1,
    //     F2: Fn(&mut Parser) -> T2,
    //     F3: Fn(&mut Parser) -> T3,
    //     F4: Fn(&mut Parser) -> T4,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let b = func2(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let c = func3(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let d = func4(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     (a, b, c, d)
    // }
    // fn read_tuple_5<T1, F1, T2, F2, T3, F3, T4, F4, T5, F5>(
    //     &mut self,
    //     func1: F1,
    //     func2: F2,
    //     func3: F3,
    //     func4: F4,
    //     func5: F5,
    // ) -> (T1, T2, T3, T4, T5)
    // where
    //     F1: Fn(&mut Parser) -> T1,
    //     F2: Fn(&mut Parser) -> T2,
    //     F3: Fn(&mut Parser) -> T3,
    //     F4: Fn(&mut Parser) -> T4,
    //     F5: Fn(&mut Parser) -> T5,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let b = func2(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let c = func3(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let d = func4(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let e = func5(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     (a, b, c, d, e)
    // }
    // fn read_tuple_6<T1, F1, T2, F2, T3, F3, T4, F4, T5, F5, T6, F6>(
    //     &mut self,
    //     func1: F1,
    //     func2: F2,
    //     func3: F3,
    //     func4: F4,
    //     func5: F5,
    //     func6: F6,
    // ) -> (T1, T2, T3, T4, T5, T6)
    // where
    //     F1: Fn(&mut Parser) -> T1,
    //     F2: Fn(&mut Parser) -> T2,
    //     F3: Fn(&mut Parser) -> T3,
    //     F4: Fn(&mut Parser) -> T4,
    //     F5: Fn(&mut Parser) -> T5,
    //     F6: Fn(&mut Parser) -> T6,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let b = func2(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let c = func3(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let d = func4(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let e = func5(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let f = func6(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     (a, b, c, d, e, f)
    // }
    // fn read_tuple_7<T1, F1, T2, F2, T3, F3, T4, F4, T5, F5, T6, F6, T7, F7>(
    //     &mut self,
    //     func1: F1,
    //     func2: F2,
    //     func3: F3,
    //     func4: F4,
    //     func5: F5,
    //     func6: F6,
    //     func7: F7,
    // ) -> (T1, T2, T3, T4, T5, T6, T7)
    // where
    //     F1: Fn(&mut Parser) -> T1,
    //     F2: Fn(&mut Parser) -> T2,
    //     F3: Fn(&mut Parser) -> T3,
    //     F4: Fn(&mut Parser) -> T4,
    //     F5: Fn(&mut Parser) -> T5,
    //     F6: Fn(&mut Parser) -> T6,
    //     F7: Fn(&mut Parser) -> T7,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let b = func2(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let c = func3(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let d = func4(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let e = func5(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let f = func6(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let g = func7(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     (a, b, c, d, e, f, g)
    // }
    // fn read_tuple_9<T1, F1, T2, F2, T3, F3, T4, F4, T5, F5, T6, F6, T7, F7, T8, F8, T9, F9>(
    //     &mut self,
    //     func1: F1,
    //     func2: F2,
    //     func3: F3,
    //     func4: F4,
    //     func5: F5,
    //     func6: F6,
    //     func7: F7,
    //     func8: F8,
    //     func9: F9,
    // ) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9)
    // where
    //     F1: Fn(&mut Parser) -> T1,
    //     F2: Fn(&mut Parser) -> T2,
    //     F3: Fn(&mut Parser) -> T3,
    //     F4: Fn(&mut Parser) -> T4,
    //     F5: Fn(&mut Parser) -> T5,
    //     F6: Fn(&mut Parser) -> T6,
    //     F7: Fn(&mut Parser) -> T7,
    //     F8: Fn(&mut Parser) -> T8,
    //     F9: Fn(&mut Parser) -> T9,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let b = func2(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let c = func3(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let d = func4(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let e = func5(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let f = func6(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let g = func7(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let h = func8(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let i = func9(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     (a, b, c, d, e, f, g, h, i)
    // }
    // fn read_tuple_13<
    //     T1,
    //     F1,
    //     T2,
    //     F2,
    //     T3,
    //     F3,
    //     T4,
    //     F4,
    //     T5,
    //     F5,
    //     T6,
    //     F6,
    //     T7,
    //     F7,
    //     T8,
    //     F8,
    //     T9,
    //     F9,
    //     T10,
    //     F10,
    //     T11,
    //     F11,
    //     T12,
    //     F12,
    //     T13,
    //     F13,
    // >(
    //     &mut self,
    //     func1: F1,
    //     func2: F2,
    //     func3: F3,
    //     func4: F4,
    //     func5: F5,
    //     func6: F6,
    //     func7: F7,
    //     func8: F8,
    //     func9: F9,
    //     func10: F10,
    //     func11: F11,
    //     func12: F12,
    //     func13: F13,
    // ) -> (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
    // where
    //     F1: Fn(&mut Parser) -> T1,
    //     F2: Fn(&mut Parser) -> T2,
    //     F3: Fn(&mut Parser) -> T3,
    //     F4: Fn(&mut Parser) -> T4,
    //     F5: Fn(&mut Parser) -> T5,
    //     F6: Fn(&mut Parser) -> T6,
    //     F7: Fn(&mut Parser) -> T7,
    //     F8: Fn(&mut Parser) -> T8,
    //     F9: Fn(&mut Parser) -> T9,
    //     F10: Fn(&mut Parser) -> T10,
    //     F11: Fn(&mut Parser) -> T11,
    //     F12: Fn(&mut Parser) -> T12,
    //     F13: Fn(&mut Parser) -> T13,
    // {
    //     self.read_open_paren();
    //     self.skip_whitespace();
    //     let a = func1(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let b = func2(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let c = func3(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let d = func4(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let e = func5(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let f = func6(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let g = func7(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let h = func8(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let i = func9(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let j = func10(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let k = func11(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let l = func12(self);
    //     self.skip_whitespace();
    //     self.read_comma();
    //     self.skip_whitespace();
    //     let m = func13(self);
    //     self.skip_whitespace();
    //     self.read_close_paren();
    //     (a, b, c, d, e, f, g, h, i, j, k, l, m)
    // }



    // fn read_label(&mut self) -> String {
    //     self.read_string()
    // }
    // fn read_count_measure(&mut self) -> CountMeasure {
    //     let (iden, val) = self.read_united_float();
    //     if iden != "COUNT_MEASURE" { panic!("iden {} should be COUNT_MEASURE", iden); }
    //     return CountMeasure(val);
    // }
    // fn read_length_measure(&mut self) -> LengthMeasure {
    //     let (iden, val) = self.read_united_float();
    //     if iden != "LENGTH_MEASURE" { panic!("iden {} should be LENGTH_MEASURE", iden); }
    //     return LengthMeasure(val);
    // }
    // fn read_id(&mut self) -> Id {
    //     Id(self.read_id())
    // }
    // fn read_id_vector(&mut self) -> Vec<Id> {
    //     self.read_vector(Parser::read_id)
    // }
    // fn read_id<T>(&mut self) -> Id<T> {
    //     Id::<T>::new(self.read_id())
    // }
    // fn read_id_vector<T>(&mut self) -> Vec<Id<T>> {
    //     self.read_vector(Parser::read_id::<T>)
    // }
}

// fn parse_header_func(iden: &str, parser: &mut Parser) {
//     parser.skip_whitespace();
//     match iden {
//         "FILE_DESCRIPTION" => {
//             let (description, implementation_level) =
//                 parser.read_tuple_2(Parser::read_string_vector, Parser::read_string);
//         }
//         "FILE_NAME" => {
//             let (
//                 name,
//                 time_stamp,
//                 author,
//                 organization,
//                 preprocessor_version,
//                 originating_system,
//                 authorisation,
//             ) = parser.read_tuple_7(
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_string_vector,
//                 Parser::read_string_vector,
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_string,
//             );
//         }
//         "FILE_SCHEMA" => {
//             parser.read_tuple_1(Parser::read_string_vector);
//         }
//         _ => panic!("unkown function >>{}<<", iden),
//     }
// }

// fn parse_data_tuple(parser: &mut Parser) {
//     parser.read_open_paren();
//     parser.skip_whitespace();
//     loop {
//         if parser.peek_char() == ')' {
//             parser.read_close_paren();
//             break;
//         }
//         let iden = parser.read_identifier();
//         parser.skip_whitespace();
//         parse_data_tuple_func(&iden, parser);
//         if parser.peek_char() == ')' {
//             parser.read_close_paren();
//             break;
//         }
//         parser.skip_expect_whitespace()
//     }
// }

// fn parse_data_tuple_func(iden: &str, parser: &mut Parser) {
//     match iden {
//         "B_SPLINE_CURVE" => {
//             parser.read_tuple_5(
//                 Parser::read_int,
//                 Parser::read_id_vector,
//                 Parser::read_unit,
//                 Parser::read_bool_literal,
//                 Parser::read_bool_literal,
//             );
//         }
//         "B_SPLINE_CURVE_WITH_KNOTS" => {
//             parser.read_tuple_3(
//                 Parser::read_int_vector,
//                 Parser::read_float_vector,
//                 Parser::read_unit,
//             );
//         }
//         "B_SPLINE_SURFACE" => {
//             parser.read_tuple_7(
//                 Parser::read_int,
//                 Parser::read_int,
//                 Parser::read_id_vector_vector,
//                 Parser::read_unit,
//                 Parser::read_bool_literal,
//                 Parser::read_bool_literal,
//                 Parser::read_bool_literal,
//             );
//         }
//         "B_SPLINE_SURFACE_WITH_KNOTS" => {
//             parser.read_tuple_5(
//                 Parser::read_int_vector,
//                 Parser::read_int_vector,
//                 Parser::read_float_vector,
//                 Parser::read_float_vector,
//                 Parser::read_unit,
//             );
//         }
//         "BOUNDED_CURVE" => {
//             parser.read_tuple_0();
//         }
//         "BOUNDED_SURFACE" => {
//             parser.read_tuple_0();
//         }
//         "CURVE" => {
//             parser.read_tuple_0();
//         }
//         "GEOMETRIC_REPRESENTATION_CONTEXT" => {
//             parser.read_tuple_1(Parser::read_int);
//         }
//         "GEOMETRIC_REPRESENTATION_ITEM" => {
//             parser.read_tuple_0();
//         }
//         "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT" => {
//             parser.read_tuple_1(Parser::read_id_vector);
//         }
//         "GLOBAL_UNIT_ASSIGNED_CONTEXT" => {
//             parser.read_tuple_1(Parser::read_id_vector);
//         }
//         "LENGTH_UNIT" => {
//             parser.read_tuple_0();
//         }
//         "NAMED_UNIT" => {
//             parser.read_tuple_1(Parser::read_star);
//         }
//         "PLANE_ANGLE_UNIT" => {
//             parser.read_tuple_0();
//         }
//         "RATIONAL_B_SPLINE_CURVE" => {
//             parser.read_tuple_1(Parser::read_float_vector);
//         }
//         "RATIONAL_B_SPLINE_SURFACE" => {
//             parser.read_tuple_1(Parser::read_float_vector_vector);
//         }
//         "REPRESENTATION_CONTEXT" => {
//             parser.read_tuple_2(Parser::read_string, Parser::read_string);
//         }
//         "REPRESENTATION_ITEM" => {
//             parser.read_tuple_1(Parser::read_string);
//         }
//         "REPRESENTATION_RELATIONSHIP" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_id,
//             );
//         }
//         "REPRESENTATION_RELATIONSHIP_WITH_TRANSFORMATION" => {
//             parser.read_tuple_1(Parser::read_id);
//         }
//         "SI_UNIT" => {
//             parser.read_tuple_2(Parser::read_unit_or_dollar, Parser::read_unit);
//         }
//         "SHAPE_REPRESENTATION_RELATIONSHIP" => {
//             parser.read_tuple_0();
//         }
//         "SOLID_ANGLE_UNIT" => {
//             parser.read_tuple_0();
//         }
//         "SURFACE" => {
//             parser.read_tuple_0();
//         }
//         _ => panic!("Unkown data tuple function >>{}<<", iden),
//     };
// }

// // fn make_id_set<T, const L: usize, const U: usize>(vec: Vec<usize>) -> Set::<Id<T>, L, U> {
// //     if vec.len() < L || vec.len() > U { panic!("vec of size {} is in [{}, {}]", vec.len(), L, U)}
// //     Set::<Id<T>, L, U >(vec.iter().map(|&x| Id::<T>::new(x)).collect::<Vec<_>>())
// // }
// // fn make_id_list<T, const L: usize, const U: usize>(vec: Vec<usize>) -> List::<Id<T>, L, U> {
// //     if vec.len() < L || vec.len() > U { panic!("vec of size {} is in [{}, {}]", vec.len(), L, U)}
// //     List::<Id<T>, L, U >(vec.iter().map(|&x| Id::<T>::new(x)).collect::<Vec<_>>())
// // }
// // fn make_id_unique_list<T, const L: usize, const U: usize>(vec: Vec<usize>) -> UniqueList::<Id<T>, L, U> {
// //     if vec.len() < L || vec.len() > U { panic!("vec of size {} is in [{}, {}]", vec.len(), L, U)}
// //     // TODO check uniqueness
// //     UniqueList::<Id<T>, L, U >(vec.iter().map(|&x| Id::<T>::new(x)).collect::<Vec<_>>())
// // }

// // fn make_checked_list<T, const L: usize, const U: usize>(vec: Vec<T>) -> List::<T, L, U> {
// //     if vec.len() < L || vec.len() > U { panic!("vec of size {} is in [{}, {}]", vec.len(), L, U)}
// //     List::<T, L, U >(vec)
// // }

// fn parse_data_func(iden: &str, parser: &mut Parser) -> Entity<String> {
//     use ap214::Entity::*;
//     parser.skip_whitespace();
//     match iden {
//         "ADVANCED_BREP_SHAPE_REPRESENTATION" => {
//             let (name, items, context_of_items) = parser.read_tuple_3(
//                 Parser::read_label,
//                 Parser::read_id_vector, //::<RepresentationItem>,
//                 Parser::read_id, //::<NullIdType>,
//             );
//             AdvancedBrepShapeRepresentation(name, items, context_of_items);
//         }
//         "ADVANCED_FACE" => {
//             let (name, bounds, face_geometry, same_sense) = parser.read_tuple_4(
//                 Parser::read_label,
//                 Parser::read_id_vector, //::<FaceBound>,
//                 Parser::read_id, //::<Surface>,
//                 Parser::read_bool_literal,
//             );
//             AdvancedFace(name, bounds, face_geometry, same_sense);
//         }
//         "APPLICATION_CONTEXT" => {
//             let name = parser.read_tuple_1(Parser::read_label);
//             ApplicationContext(name);
//         }
//         "APPLICATION_PROTOCOL_DEFINITION" => {
//             let (s1, s2, int, id) = parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_int,
//                 Parser::read_id,
//             );
//             ApplicationProtocolDefinition(s1, s2, int as u32, id);
//         }
//         "AXIS2_PLACEMENT_3D" => {
//             let (name, location, dir1, dir2) = parser.read_tuple_4(
//                 Parser::read_label,
//                 Parser::read_id, //::<CartesianPoint>,
//                 Parser::read_id, //:<Direction>,
//                 Parser::read_id, //::<Direction>,
//             );
//             Axis2Placement3d(name, location, dir1, dir2);
//         }
//         "B_SPLINE_CURVE_WITH_KNOTS" => {
//             parser.read_tuple_9(
//                 Parser::read_string,
//                 Parser::read_int,
//                 Parser::read_id_vector,
//                 Parser::read_unit, /* actual enum? */
//                 Parser::read_bool_literal,
//                 Parser::read_bool_literal,
//                 Parser::read_int_vector,
//                 Parser::read_float_vector,
//                 Parser::read_unit, /* actual enum? */
//             );
//         }
//         "B_SPLINE_SURFACE_WITH_KNOTS" => {
//             parser.read_tuple_13(
//                 Parser::read_string,
//                 Parser::read_int,
//                 Parser::read_int,
//                 Parser::read_id_vector_vector,
//                 Parser::read_unit, /* actual enum? */
//                 Parser::read_bool_literal,
//                 Parser::read_bool_literal,
//                 Parser::read_bool_literal,
//                 Parser::read_int_vector,
//                 Parser::read_int_vector,
//                 Parser::read_float_vector,
//                 Parser::read_float_vector,
//                 Parser::read_unit, /* actual enum? */
//             );
//         }
//         "BREP_WITH_VOIDS" => {
//             parser.read_tuple_3(
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_id_vector,
//             );
//         }
//         "CARTESIAN_POINT" => {
//             let (name, coords) = parser.read_tuple_2(Parser::read_label, Parser::read_float_vector);
//             if coords.len() != 3 { panic!("wrong size tuple"); }
//             let mut it = coords.into_iter();
//             let coords = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
//             CartesianPoint(name, coords);
//         }
//         "CIRCLE" => {
//             let (name, position, radius) = parser.read_tuple_3(
//                 Parser::read_label,
//                 Parser::read_id, //::<Axis2Placement>,
//                 Parser::read_float);
//             if radius <= 0.0 { panic!("raidus {} is not > 0", radius) }
//             Circle(name, position, radius);
//         }
//         "CLOSED_SHELL" => {
//             let (name, cfs_faces) = parser.read_tuple_2(Parser::read_label,
//                 Parser::read_id_vector, //::<Face>
//                 );
//             ClosedShell(name, cfs_faces);
//         }
//         "COLOUR_RGB" => {
//             let (name, red, green, blue) = parser.read_tuple_4(
//                 Parser::read_label,
//                 Parser::read_float,
//                 Parser::read_float,
//                 Parser::read_float,
//             );
//             ColourRgb(name, red, green, blue);
//         }
//         "CONICAL_SURFACE" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_float,
//                 Parser::read_float,
//             );
//         }
//         "CONTEXT_DEPENDENT_SHAPE_REPRESENTATION" => {
//             parser.read_tuple_2(Parser::read_id, Parser::read_id);
//         }
//         "CURVE_STYLE" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_united_float,
//                 Parser::read_id,
//             );
//         }
//         "CYLINDRICAL_SURFACE" => {
//             let (name, position, radius) = parser.read_tuple_3(Parser::read_label,
//                 Parser::read_id, //::<Axis2Placement3D>,
//                 Parser::read_float);
//             if radius <= 0.0 { panic!("raidus {} is not > 0", radius) }
//             CylindricalSurface(name, position, radius);
//         }
//         "DERIVED_UNIT" => {
//             parser.read_tuple_1(Parser::read_id_vector);
//         }
//         "DERIVED_UNIT_ELEMENT" => {
//             parser.read_tuple_2(Parser::read_id, Parser::read_float);
//         }
//         "DESCRIPTIVE_REPRESENTATION_ITEM" => {
//             parser.read_tuple_2(Parser::read_string, Parser::read_string);
//         }
//         "DIRECTION" => {
//             let (name, dir_ratios) = parser.read_tuple_2(Parser::read_label, Parser::read_float_vector);
//             if dir_ratios.len() != 3 { panic!("wrong size tuple"); }
//             let mut it = dir_ratios.into_iter();
//             let dir_ratios = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());
//             Direction(name, dir_ratios);
//         }
//         "DRAUGHTING_PRE_DEFINED_COLOUR" => {
//             parser.read_tuple_1(Parser::read_string);
//         }
//         "DRAUGHTING_PRE_DEFINED_CURVE_FONT" => {
//             parser.read_tuple_1(Parser::read_string);
//         }
//         "EDGE_CURVE" => {
//             let (name, edge_start, edge_end, edge_geometry, same_sense) = parser.read_tuple_5(
//                 Parser::read_label,
//                 Parser::read_id, //::<Vertex>,
//                 Parser::read_id, //::<Vertex>,
//                 Parser::read_id, //::<Curve>,
//                 Parser::read_bool_literal,
//             );
//             EdgeCurve(name, edge_start, edge_end, edge_geometry, same_sense);
//         }
//         "EDGE_LOOP" => {
//             let (name, edge_list) = parser.read_tuple_2(Parser::read_label,
//              Parser::read_id_vector, //::<OrientedEdge>
//              );
//             EdgeLoop(name, edge_list);
//         }
//         "ELLIPSE" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_float,
//                 Parser::read_float,
//             );
//         }
//         "FACE_BOUND" => {
//             let (name, bound, orientation) = parser.read_tuple_3(
//                 Parser::read_label,
//                 Parser::read_id, //::<Loop>,
//                 Parser::read_bool_literal,
//             );
//             FaceBound(name, bound, orientation);
//         }
//         "FILL_AREA_STYLE" => {
//             let (name, fill_stypes) = parser.read_tuple_2(Parser::read_label,
//                 Parser::read_id_vector, //::<FillStyleSelect>
//                 );
//             FillAreaStyle(name, fill_stypes);
//         }
//         "FILL_AREA_STYLE_COLOUR" => {
//             let (name, fill_color) = parser.read_tuple_2(Parser::read_label,
//              Parser::read_id, //::<Colour>
//              );
//             FillAreaStyleColour(name, fill_color);
//         }
//         "ITEM_DEFINED_TRANSFORMATION" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_id,
//             );
//         }
//         "GEOMETRIC_CURVE_SET" => {
//             parser.read_tuple_2(Parser::read_string, Parser::read_id_vector);
//         }
//         "LINE" => {
//             let (name, pnt, dir) = parser.read_tuple_3(Parser::read_label,
//                 Parser::read_id, //::<CartesianPoint>,
//                 Parser::read_id, //::<Vector>
//                 );
//             Line(name, pnt, dir);
//         }
//         "MANIFOLD_SOLID_BREP" => {
//             let (name, outer) = parser.read_tuple_2(
//                 Parser::read_label,
//                 Parser::read_id, //::<ClosedShell>
//                 );
//             ManifoldSolidBrep(name, outer);
//         }
//         "MANIFOLD_SURFACE_SHAPE_REPRESENTATION" => {
//             parser.read_tuple_3(
//                 Parser::read_string,
//                 Parser::read_id_vector,
//                 Parser::read_id,
//             );
//         }
//         "MEASURE_REPRESENTATION_ITEM" => {
//             parser.read_tuple_3(
//                 Parser::read_string,
//                 Parser::read_united_float,
//                 Parser::read_id,
//             );
//         }
//         "MECHANICAL_DESIGN_GEOMETRIC_PRESENTATION_REPRESENTATION" => {
//             let (name, items, context_of_items) = parser.read_tuple_3(
//                 Parser::read_label,
//                 Parser::read_id_vector, //::<RepresentationItem>,
//                 Parser::read_id, //::<RepresentationContext>,
//             );
//             MechanicalDesignGeometricPresentationRepresentation(name, items, context_of_items);
//         }
//         "NEXT_ASSEMBLY_USAGE_OCCURRENCE" => {
//             parser.read_tuple_6(
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_id,
//                 Parser::read_dollar,
//             );
//         }
//         "ORIENTED_EDGE" => {
//             let (name, _, _, edge_element, orientation) = parser.read_tuple_5(
//                 Parser::read_label,
//                 Parser::read_star,
//                 Parser::read_star,
//                 Parser::read_id, //::<Vertex>,
//                 Parser::read_bool_literal,
//             );
//             OrientedEdge(name, edge_element, orientation);
//         }
//         "OPEN_SHELL" => {
//             parser.read_tuple_2(Parser::read_string, Parser::read_id_vector);
//         }
//         "ORIENTED_CLOSED_SHELL" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_star,
//                 Parser::read_id,
//                 Parser::read_bool_literal,
//             );
//         }
//         "OVER_RIDING_STYLED_ITEM" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_id_vector,
//                 Parser::read_id,
//                 Parser::read_id,
//             );
//         }
//         "PLANE" => {
//             let (name, position) = parser.read_tuple_2(Parser::read_label,
//                 Parser::read_id, //::<Axis2Placement3D>
//                 );
//             Plane(name, position);
//         }
//         "PRESENTATION_LAYER_ASSIGNMENT" => {
//             parser.read_tuple_3(
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_id_vector,
//             );
//         }
//         "PRESENTATION_STYLE_ASSIGNMENT" => {
//             let styles = parser.read_tuple_1(
//                 Parser::read_id_vector, //::<PresentationStyleSelect>
//                 );
//             PresentationStyleAssignment::<String>(styles);
//         }
//         "PRESENTATION_STYLE_BY_CONTEXT" => {
//             parser.read_tuple_2(Parser::read_id_vector, Parser::read_id);
//         }
//         "PRODUCT" => {
//             let (s1, s2, s3, ids) = parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_id_vector,
//             );
//             Product(s1, s2, s3, ids);
//         }
//         "PRODUCT_CATEGORY" => {
//             let (s1, s2) = parser.read_tuple_2(Parser::read_string, Parser::read_string);
//             ProductCategory(s1, s2);
//         }
//         "PRODUCT_CONTEXT" => {
//             let (s1, id, s2) = parser.read_tuple_3(Parser::read_string, Parser::read_id, Parser::read_string);
//             ProductContext(s1, id, s2);
//         }
//         "PRODUCT_DEFINITION" => {
//             let (name, s1, id1, id2) = parser.read_tuple_4(
//                 Parser::read_label,
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_id,
//             );
//             ProductDefinition(name, s1, id1, id2);
//         }
//         "PRODUCT_DEFINITION_CONTEXT" => {
//             let (name, id, s) = parser.read_tuple_3(Parser::read_label, Parser::read_id, Parser::read_string);
//             ProductDefinitionContext(name, id, s);
//         }
//         "PRODUCT_DEFINITION_FORMATION" => {
//             parser.read_tuple_3(Parser::read_string, Parser::read_string, Parser::read_id);
//         }
//         "PRODUCT_DEFINITION_FORMATION_WITH_SPECIFIED_SOURCE" => {
//             let (name, s1, id1, source) = parser.read_tuple_4(
//                 Parser::read_label,
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_unit,
//             );
//             let source = match &source[..] {
//                 "MADE" => Source::Made,
//                 "BOUGHT"=> Source::Bought,
//                 "NOT_KNOWN"=> Source::NotKnown,
//                 _ => panic!("invalid enum value of >>{}<<", source)
//             };
//             ProductDefinitionFormationWithSpecifiedSource(name, s1, id1, source);
//         }
//         "PRODUCT_DEFINITION_SHAPE" => {
//             let (name, s1, id) = parser.read_tuple_3(Parser::read_label, Parser::read_string, Parser::read_id);
//             ProductDefinitionShape(name, s1, id);
//         }
//         "PRODUCT_RELATED_PRODUCT_CATEGORY" => {
//             let (name, s1, ids) = parser.read_tuple_3(
//                 Parser::read_label,
//                 Parser::read_string_or_dollar,
//                 Parser::read_id_vector,
//             );
//             // let s1 = s1.unwrap();  // TODO deal with null case
//             // ProductRelatedProductCategory(name, s1, ids);
//         }
//         "PROPERTY_DEFINITION" => {
//             let (name, description, definition) = parser.read_tuple_3(Parser::read_label, Parser::read_string,
//                 Parser::read_id, //::<CharacterizedDefinition>
//                 );
//             PropertyDefinition(name, description, definition);
//         }
//         "PROPERTY_DEFINITION_REPRESENTATION" => {
//             let (prop_definition_id, used_representation) = parser.read_tuple_2(
//                 Parser::read_id, //::<RepresentedDefinition>,
//                 Parser::read_id, //::<Representation>
//                 );
//             PropertyDefinitionRepresentation::<String>(prop_definition_id, used_representation);
//         }
//         "REPRESENTATION" => {
//             let (name, items, context_of_items) = parser.read_tuple_3(
//                 Parser::read_string_or_dollar,
//                 Parser::read_id_vector, // ::<RepresentationItem>,
//                 Parser::read_id_or_dollar,
//             );
//             //let name = name.unwrap(); // TODO handle null
//             //let id = Id(context_of_items.unwrap()); // TODO handle null
//             //Representation(name, items, id);
//         }
//         "SHAPE_ASPECT" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_bool_literal,
//             );
//         }
//         "SHAPE_DEFINITION_REPRESENTATION" => {
//             let (definition, used_representation) = parser.read_tuple_2(
//                 Parser::read_id, //::<RepresentedDefinition>,
//                 Parser::read_id, //::<Representation>
//             );
//             ShapeDefinitionRepresentation::<String>(definition, used_representation);
//         }
//         "SHAPE_REPRESENTATION" => {
//             let (name, items, context_of_items) = parser.read_tuple_3(
//                 Parser::read_label,
//                 Parser::read_id_vector, //::<RepresentationItem>,
//                 Parser::read_id, //::<NullIdType2>,
//             );
//             ShapeRepresentation(name, items, context_of_items);
//         }
//         "SHAPE_REPRESENTATION_RELATIONSHIP" => {
//             let (name, description, rep_1, rep_2) = parser.read_tuple_4(
//                 Parser::read_label,
//                 Parser::read_string,
//                 Parser::read_id, //::<Representation>,
//                 Parser::read_id, //::<Representation>,
//             );
//             ShapeRepresentationRelationship(name, description, rep_1, rep_2);
//         }
//         "SHELL_BASED_SURFACE_MODEL" => {
//             parser.read_tuple_2(Parser::read_string, Parser::read_id_vector);
//         }
//         "SPHERICAL_SURFACE" => {
//             parser.read_tuple_3(Parser::read_string, Parser::read_id, Parser::read_float);
//         }
//         "SURFACE_OF_LINEAR_EXTRUSION" => {
//             parser.read_tuple_3(Parser::read_string, Parser::read_id, Parser::read_id);
//         }
//         "SURFACE_SIDE_STYLE" => {
//             let (name, styles) = parser.read_tuple_2(Parser::read_label, Parser::read_id_vector, //::<SurfaceStyleElementSelect>
//                 );
//             SurfaceSideStyle(name, styles);
//         }
//         "SURFACE_STYLE_FILL_AREA" => {
//             let fill_area = parser.read_tuple_1(Parser::read_id, //::<FillAreaStyle>
//                 );
//             SurfaceStyleFillArea::<String>(fill_area);
//         }
//         "SURFACE_STYLE_USAGE" => {
//             let (side, style) = parser.read_tuple_2(
//                 Parser::read_unit, /* actual enum? */
//                 Parser::read_id, //::<SurfaceSideStyleSelect>,
//             );
//             let side = match &side[..] {
//                 "POSITIVE" => SurfaceSide::Positive,
//                 "NEGATIVE" => SurfaceSide::Negative,
//                 "BOTH" => SurfaceSide::Both,
//                 _ => panic!("invalid Surface Side value {}", side)
//             };
//             SurfaceStyleUsage::<String>(side, style);
//         }
//         "STYLED_ITEM" => {
//             let (name, styles, item) = parser.read_tuple_3(
//                 Parser::read_label,
//                 Parser::read_id_vector, //::<PresentationStyleAssignment>,
//                 Parser::read_id, //::<RepresentationItem>,
//             );
//             StyledItem(name, styles, item);
//         }
//         "TRIMMED_CURVE" => {
//             parser.read_tuple_6(
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_id_united_float_pair,
//                 Parser::read_id_united_float_pair,
//                 Parser::read_bool_literal,
//                 Parser::read_unit, /*special enum?*/
//             );
//         }
//         "TOROIDAL_SURFACE" => {
//             parser.read_tuple_4(
//                 Parser::read_string,
//                 Parser::read_id,
//                 Parser::read_float,
//                 Parser::read_float,
//             );
//         }
//         "VALUE_REPRESENTATION_ITEM" => {
//             let (name, value_component) = parser.read_tuple_2(Parser::read_label, Parser::read_count_measure);
//             ValueRepresentationItem(name, value_component);
//         }
//         "VERTEX_POINT" => {
//             let (name, vertex_geometry) = parser.read_tuple_2(Parser::read_label, Parser::read_id, //::<Point>
//                 );
//             VertexPoint(name, vertex_geometry);
//         }
//         "VECTOR" => {
//             let (name, orientation, magnitude) = parser.read_tuple_3(Parser::read_label, Parser::read_id, //::<Direction>,
//              Parser::read_float);
//             Vector(name, orientation, magnitude);
//         }
//         "UNCERTAINTY_MEASURE_WITH_UNIT" => {
//             let (len, id, s1, s2) = parser.read_tuple_4(
//                 Parser::read_length_measure,
//                 Parser::read_id,
//                 Parser::read_string,
//                 Parser::read_string,
//             );
//             UncertaintyMeasureWithUnit(len, id, s1, s2);
//         }
//         _ => { panic!("Unkown data function >>{}<<", iden) }
//     };
//     Vector("".to_string(), Id(0), 3.0)
// }

// fn parse_file(filename: String) -> HashMap<usize, Entity<String>> {
//     let file = File::open(filename).expect("file opens");
//     let cursor = io::BufReader::new(file); //.expect("curser is creatable");
//     let mut parser = Parser {
//         stream: PeekableBufReader::new(cursor),
//     };

//     let iso_str = parser.read_up_to(';');
//     parser.read_semicolon();
//     parser.skip_whitespace();
//     println!("{}", iso_str);

//     // READ IN HEADER SETUP
//     let iden = parser.read_identifier();
//     if iden != "HEADER" {
//         panic!("expected 'HEADER'");
//     }
//     parser.read_semicolon();
//     parser.skip_whitespace();

//     // READ IN HEADER FUNCTIONS
//     loop {
//         let iden = parser.read_identifier();
//         if iden == "ENDSEC" {
//             parser.read_semicolon();
//             parser.skip_whitespace();
//             break;
//         }
//         parse_header_func(&iden, &mut parser);
//         parser.read_semicolon();
//         parser.skip_whitespace();
//     }

//     // READ IN DATA SETUP
//     let iden = parser.read_identifier();
//     if iden != "DATA" {
//         panic!("expected 'DATA'");
//     }
//     parser.read_semicolon();
//     parser.skip_whitespace();

//     let mut mp = HashMap::new();

//     // READ IN DATA FUNCTIONS
//     loop {
//         match parser.peek_char() {
//             '#' => {
//                 let idx = parser.read_id();
//                 parser.skip_whitespace();
//                 parser.read_equal_sign();
//                 parser.skip_whitespace();
//                 match parser.peek_char() {
//                     '(' => parse_data_tuple(&mut parser),
//                     'A'..='Z' | 'a'..='z' => {
//                         let iden = parser.read_identifier();
//                         let entity = parse_data_func(&iden, &mut parser);
//                         mp.insert(idx, entity);
//                     }
//                     _ => panic!("unexpected character >>{}<<", parser.peek_char()),
//                 };
//                 parser.read_semicolon();
//                 parser.skip_whitespace();
//             }
//             _ => {
//                 let iden = parser.read_identifier();
//                 if iden != "ENDSEC" {
//                     panic!("unexpected line in data section >>{}<<", iden)
//                 }
//                 parser.read_semicolon();
//                 parser.skip_whitespace();
//                 break;
//             }
//         }
//     }
//     println!("done loop");

//     let end_iso_str = parser.read_up_to(';');
//     parser.read_semicolon();
//     parser.skip_whitespace();
//     println!("{}", end_iso_str);

//     parser.read_eof();

//     mp
// }

// fn main() {
//     // File hosts must exist in current path before this produces output

//     // println!("hi");
//     let filename = "/Users/Henry Heffan/Desktop/foxtrot/HOLEWIZARD.STEP";
//     let filename = "/Users/Henry Heffan/Desktop/foxtrot/KondoMotherboard_RevB_full.STEP";
//     parse_file(filename.to_string());
// }
