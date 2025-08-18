use pyo3::prelude::*;

// # performance note: Local benmcharking (so take it with a big grain of salt!)
// # on this indicates that it is is on average slightly slower than
// # checking two separate patterns, but the standard deviation is smaller with
// # one pattern. The time difference between the two was ~2 std deviations, which
// # is small enough that I've just chosen the more readable option.
// _HAS_RENDER_CHARS_PAT = re.compile(r"({[{%#]|[#}%]})")
use regex::Regex;

fn has_render_chars_pattern() -> Regex {
    Regex::new(r"({[{%#]|[#}%]})").unwrap()
}

pub fn get_rendered(template: &str, context: Option<&Bound<'_, PyAny>>) -> String {
    let has_render_chars = has_render_chars_pattern().is_match(template);

    if !has_render_chars {
        return template.to_string();
    } else {
        unimplemented!()
    }
}
