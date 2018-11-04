use regex::Regex;

pub trait Matcher {
    fn match_text(&self, text_to_test: &str) -> bool;
}

impl<'a> Matcher {
    pub fn of_text(text_to_search: &'a str) -> TextMatcher<'a> {
        TextMatcher { text_to_search }
    }

    pub fn not(matcher: Box<Matcher + 'a>) -> NotMatcher<'a> {
        NotMatcher { matcher }
    }

    pub fn of_regex(regex: &'a str) -> RegexMatcher {
        RegexMatcher { regex: Regex::new(regex).unwrap() }
    }
}

pub struct TextMatcher<'a> {
    text_to_search: &'a str
}

pub struct NotMatcher<'a> {
    matcher: Box<Matcher + 'a>
}

pub struct RegexMatcher {
    regex: Regex
}

impl<'a> Matcher for TextMatcher<'a> {
    fn match_text(&self, text_to_test: &str) -> bool {
        text_to_test.contains(self.text_to_search)
    }
}

impl<'a> Matcher for NotMatcher<'a> {
    fn match_text(&self, text_to_test: &str) -> bool {
        !self.matcher.match_text(text_to_test)
    }
}

impl Matcher for RegexMatcher {
    fn match_text(&self, text_to_test: &str) -> bool {
        self.regex.is_match(text_to_test)
    }
}