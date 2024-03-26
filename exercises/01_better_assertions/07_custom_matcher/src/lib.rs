use googletest::{
    description::Description,
    matcher::{Matcher, MatcherResult},
};
use http::StatusCode;

pub fn is_redirect() -> impl Matcher<ActualT = StatusCode> {
    StatusCodeMatcher {}
}

struct StatusCodeMatcher {}

impl Matcher for StatusCodeMatcher {
    type ActualT = StatusCode;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
        actual.is_redirection().into()
    }

    fn describe(
        &self,
        matcher_result: googletest::matcher::MatcherResult,
    ) -> googletest::description::Description {
        "is a redirection status code".into()
    }

    fn explain_match(&self, actual: &Self::ActualT) -> Description {
        "which isn't a redirection status code".into()
    }
}

#[cfg(test)]
mod tests {
    use crate::is_redirect;
    use googletest::assert_that;
    use http::StatusCode;

    #[test]
    fn success() {
        assert_that!(StatusCode::MOVED_PERMANENTLY, is_redirect());
    }

    #[test]
    fn failure() {
        assert_that!(StatusCode::OK, is_redirect());
    }
}
