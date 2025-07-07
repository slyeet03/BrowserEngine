use crate::dom::{self, Node};

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    //read the next character without consuming it
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    //check what the next character start with the given string
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    //if given string is found at current pos, consume it else panic
    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!("Expected {:?} at byte {} it was not found", s, self.pos);
        }
    }

    //give true if all input is consumed
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    //consumes the character
    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        return c;
    }

    //consume character until given function returns false
    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    //consume and discard whitespace
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    //parse a tag or attribute name
    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    //parse a node
    fn parse_node(&mut self) -> Node {
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    //parse a text node
    fn parse_text(&mut self) -> Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    //parse a element
    fn parse_element(&mut self) -> Node {
        //opening tag
        self.expect("<");
        let tag_name = self.parse_name();
        let attrs = self.parse_attributes();
        self.expect(">");

        //contents
        let children = self.parse_nodes();

        //closing tag
        self.expect("</");
        self.expect(&tag_name.as_str());
        self.expect(">");

        return dom::elem(tag_name, attrs, children);
    }
}
