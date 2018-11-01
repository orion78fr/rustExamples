pub trait Matcher {
    fn match_text(&self, text_to_test: &str) -> bool;
}

impl<'a> Matcher {
    pub fn of_text(text_to_search: &'a str) -> TextMatcher<'a> {
        TextMatcher { text_to_search }
    }

    pub fn not(matcher: &'a Matcher) -> NotMatcher<'a> {
        NotMatcher { matcher }
    }
}

pub struct TextMatcher<'a> {
    text_to_search: &'a str
}

impl<'a> Matcher for TextMatcher<'a> {
    fn match_text(&self, text_to_test: &str) -> bool {
        text_to_test.contains(self.text_to_search)
    }
}

pub struct NotMatcher<'a> {
    matcher: &'a Matcher
}

impl<'a> Matcher for NotMatcher<'a> {
    fn match_text(&self, text_to_test: &str) -> bool {
        !self.matcher.match_text(text_to_test)
    }
}