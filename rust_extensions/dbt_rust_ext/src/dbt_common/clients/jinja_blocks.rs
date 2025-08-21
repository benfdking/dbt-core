use regex::{Match, Regex};
use std::collections::HashMap;
use std::fmt;
use std::option::Option;
use std::vec::Vec;

pub struct BlockTag {
    pub block_type_name: String,
    pub block_name: String,
    pub contents: Option<String>,
    pub full_block: Option<String>,
}

pub struct BlockIterator<'a> {
    pub tag_parser: TagIterator,
    pub warning_callback: Option<&'a dyn Fn(/* ExtractWarning */)>,
    pub current: Option<Tag>,
    pub stack: Vec<String>,
    pub last_position: isize,
}

impl<'a> BlockIterator<'a> {
    pub fn new(
        tag_iterator: TagIterator,
        warning_callback: Option<&'a dyn Fn(/* ExtractWarning */)>,
    ) -> Self {
        Self {
            tag_parser: tag_iterator,
            warning_callback,
            current: None,
            stack: Vec::new(),
            last_position: 0,
        }
    }

    pub fn is_current_end(&self, tag: &Tag) -> bool {
        tag.block_type_name.starts_with("end")
            && self.current.is_some()
            && tag.block_type_name[3..] == self.current.as_ref().unwrap().block_type_name
    }

    /// Find all top-level blocks in the data.
    pub fn find_blocks(
        &mut self,
        allowed_blocks: Option<&std::collections::HashSet<String>>,
        collect_raw_data: Option<bool>,
    ) -> Result<Vec<BlockTag>, String> {
        let mut blocks = Vec::new();

        let allowed_blocks: &std::collections::HashSet<String> =
            if let Some(blocks) = allowed_blocks {
                blocks
            } else {
                &["snapshot", "macro", "materialization", "docs"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<std::collections::HashSet<String>>()
            };

        let control_flow_tags = control_flow_tags();
        let control_flow_end_tags = control_flow_end_tags();

        for tag in &self.tag_parser.find_tags() {
            if control_flow_tags.contains_key(&tag.block_type_name) {
                self.stack.push(tag.block_type_name.clone());
            } else if control_flow_end_tags.contains_key(&tag.block_type_name) {
                let mut found: Option<String> = None;
                if self.stack.len() > 0 {
                    found = self.stack.pop();
                } else {
                    let expected = control_flow_tags.get(&tag.block_type_name).unwrap();
                    return Err(format!(
                        "Mismatched block end tag {} found in {}, expected {}",
                        tag.block_type_name,
                        self.tag_parser.linepos(Some(tag.start)),
                        expected
                    ));
                }
                let expected = control_flow_tags.get(&found.unwrap()).unwrap();
                if &tag.block_type_name != expected {
                    return Err(format!(
                        "Mismatched block end tag {} found in {}, expected {}",
                        tag.block_type_name,
                        self.tag_parser.linepos(Some(tag.start)),
                        expected
                    ));
                }
            }

            if allowed_blocks.contains(&tag.block_type_name) {
                if !self.stack.is_empty() {
                    return Err(format!(
                        "Block definition not at top level: {} found at {}",
                        tag.block_type_name,
                        self.tag_parser.linepos(Some(tag.start))
                    ));
                }
                if self.current.is_some() {
                    let current = self.current.as_ref().unwrap();
                    return Err(format!(
                        "Nested tags error: outer {} at {}, inner {} at {}",
                        current.block_type_name,
                        self.tag_parser.linepos(Some(current.start)),
                        tag.block_type_name,
                        self.tag_parser.linepos(Some(tag.start))
                    ));
                }
                if collect_raw_data.unwrap_or(false) {
                    unimplemented!()
                    // let raw_data = &self.tag_parser.text.to_string()[self.last_position..tag.start as usize];
                    // self.last_position = tag.start;
                    // if !raw_data.is_empty() {
                    // TODO: yield BlockData(raw_data) - need to implement BlockData
                    // }
                }
                self.current = Some(tag.clone());
            } else if self.is_current_end(&tag) {
                self.last_position = tag.end as isize;
                assert!(self.current.is_some());
                blocks.push(BlockTag {
                    block_type_name: self.current.as_ref().unwrap().block_type_name.clone(),
                    block_name: self.current.as_ref().unwrap().block_name.clone().unwrap(),
                    contents: None,
                    full_block: None,
                });
                self.current = None;
            } else if self.current.is_none() {
                return Err(format!(
                    "unexpected_block: Found unexpected {} block tag.",
                    self.current.as_ref().unwrap().block_type_name
                ));
            };
        }

        if self.current.is_some() {
            return Err(format!(
                "unexpected_block: Found unexpected {} block tag.",
                self.current.as_ref().unwrap().block_type_name
            ));
        }
        if collect_raw_data.unwrap_or(false) {
            unimplemented!()
            // let raw_data = &self.tag_parser.text[self.last_position..];
            // self.last_position = self.tag_parser.text.len();
            // if !raw_data.is_empty() {
            //     // TODO: yield BlockData(raw_data) - need to implement BlockData
            // }
        }

        return Ok(blocks);
    }

    pub fn lex_for_blocks(
        &mut self,
        allowed_blocks: Option<&std::collections::HashSet<String>>,
        collect_raw_data: Option<bool>,
    ) -> Vec<BlockTag> {
        let collect_raw_data = collect_raw_data.unwrap_or(true);
        self.find_blocks(allowed_blocks, Some(collect_raw_data))
            .unwrap()
    }
}

const CONTROL_FLOW_TAGS: [[&str; 2]; 2] = [["if", "endif"], ["for", "endfor"]];

fn control_flow_tags() -> HashMap<String, String> {
    let mut tags = HashMap::new();
    for tag in CONTROL_FLOW_TAGS {
        tags.insert(tag[0].to_string(), tag[1].to_string());
    }
    tags
}

fn control_flow_end_tags() -> HashMap<String, String> {
    let mut tags = HashMap::new();
    for tag in CONTROL_FLOW_TAGS {
        tags.insert(tag[1].to_string(), tag[0].to_string());
    }
    tags
}

// const NAME_PATTERN: &str = r"[A-Za-z_][A-Za-z_0-9]*";

// const COMMENT_START_PATTERN: &str = r"(?:(?P<comment_start>(\s*\{\#)))";
// const COMMENT_END_PATTERN: &str = r"(.*?)(\s*\#\})";
// const RAW_START_PATTERN: &str = r"(?:\s*\{\%\-|\{\%)\s*(?P<raw_start>(raw))\s*(?:\-\%\}\s*|\%\})";
// const EXPR_START_PATTERN: &str = r"(?P<expr_start>(\{\{\s*))";
// const EXPR_END_PATTERN: &str = r"(?P<expr_end>(\s*\}\}))";

// pub fn comment_start_pattern() -> Regex {
//     Regex::new(COMMENT_START_PATTERN).unwrap()
// }
// pub fn comment_end_pattern() -> Regex {
//     Regex::new(COMMENT_END_PATTERN).unwrap()
// }
// pub fn raw_start_pattern() -> Regex {
//     Regex::new(RAW_START_PATTERN).unwrap()
// }
// pub fn expr_start_pattern() -> Regex {
//     Regex::new(EXPR_START_PATTERN).unwrap()
// }
// pub fn expr_end_pattern() -> Regex {
//     Regex::new(EXPR_END_PATTERN).unwrap()
// }

impl BlockTag {
    pub fn new(
        block_type_name: String,
        block_name: String,
        contents: Option<String>,
        full_block: Option<String>,
    ) -> Self {
        Self {
            block_type_name,
            block_name,
            contents,
            full_block,
        }
    }

    pub fn end_block_type_name(&self) -> String {
        format!("end{}", self.block_type_name)
    }

    pub fn end_pat(&self) -> regex::Regex {
        // we don't want to use string formatting here because jinja uses most
        // of the string formatting operators in its syntax...
        let pattern = [
            r"(?P<endblock>((?:\s*\{\%\-|\{\%)\s*",
            &self.end_block_type_name(),
            r"\s*(?:\-\%\}\s*|\%\})))",
        ]
        .concat();
        regex::Regex::new(&pattern).unwrap()
    }
}

impl fmt::Display for BlockTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BlockTag({:?}, {:?})",
            self.block_type_name, self.block_name
        )
    }
}

impl fmt::Debug for BlockTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Debug, Clone)]
pub struct RegexMatch {
    haystack: String,
    start: usize,
    end: usize,
}

impl RegexMatch {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'a> From<regex::Match<'a>> for RegexMatch {
    fn from(match_: regex::Match<'a>) -> Self {
        Self {
            haystack: match_.as_str().to_string(),
            start: match_.start(),
            end: match_.end(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PositionedMatch {
    pub start_pos: usize,
    pub match_: Option<RegexMatch>,
}

impl PositionedMatch {
    pub fn new(start_pos: usize, match_: Option<regex::Match<'_>>) -> Self {
        Self {
            start_pos,
            match_: match_.map(|m| m.into()),
        }
    }
}

use std::sync::LazyLock;

// Define the regex patterns as static lazy-initialized values
static EXPR_END_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<expr_end>\}\})").unwrap());

static QUOTE_START_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"["']"#).unwrap());

static STRING_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    // Matches quoted strings with escape sequences
    Regex::new(r#"'(?:[^'\\]|\\.)*'|"(?:[^"\\]|\\.)*""#).unwrap()
});

static COMMENT_END_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"#\}").unwrap());

static TAG_CLOSE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<tag_close>%\})").unwrap());

static RAW_BLOCK_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\{%\s*raw\s*%\}.*?\{%\s*endraw\s*%\}").unwrap());

static BLOCK_START_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{%\s*(?P<block_type_name>\w+)(?:\s+(?P<block_name>\w+))?").unwrap()
});

static COMMENT_START_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<comment_start>\{#)").unwrap());

static EXPR_START_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<expr_start>\{\{)").unwrap());

#[derive(Debug, Clone)]
pub struct Tag {
    pub block_type_name: String,
    pub block_name: Option<String>,
    pub start: usize,
    pub end: usize,
}

// Simple struct to hold match bounds instead of the actual Match object
#[derive(Debug, Clone)]
struct MatchInfo {
    start: usize,
    end: usize,
    text: String,
}

#[derive(Debug)]
pub enum TagIteratorError {
    UnexpectedEOF(String, String),
    Internal(String),
}

impl std::fmt::Display for TagIteratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagIteratorError::UnexpectedEOF(expected, remaining) => {
                write!(
                    f,
                    "Unexpected EOF while looking for {}, remaining: {}",
                    expected, remaining
                )
            }
            TagIteratorError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for TagIteratorError {}

pub struct TagIterator {
    text: String,
    pos: usize,
    // Cache stores position and optional match bounds
    past_matches: HashMap<String, (usize, Option<(usize, usize)>)>,
}

impl TagIterator {
    pub fn new(text: String) -> Self {
        Self {
            text,
            pos: 0,
            past_matches: HashMap::new(),
        }
    }

    pub fn linepos(&self, end: Option<usize>) -> String {
        let end_val = end.unwrap_or(self.pos);
        let text_slice = &self.text[..end_val];

        // Find the last newline position
        let last_line_start = text_slice.rfind('\n').map(|n| n + 1).unwrap_or(0);

        // Count newlines for line number (1-indexed)
        let line_number = text_slice.chars().filter(|&c| c == '\n').count() + 1;

        format!("{}:{}", line_number, end_val - last_line_start)
    }

    pub fn advance(&mut self, new_position: usize) {
        self.pos = new_position;
    }

    pub fn rewind(&mut self, amount: usize) {
        self.pos = self.pos - amount;
    }

    // Return match bounds instead of Match object
    fn search(&mut self, pattern: &Regex, pattern_key: &str) -> Option<MatchInfo> {
        // Check cached search
        let cached = self.past_matches.get(pattern_key).cloned();

        let match_result = if let Some((cached_pos, cached_match)) = cached {
            if cached_pos > self.pos {
                // Cached search is ahead, need to re-search
                pattern.find_at(&self.text, self.pos)
            } else if let Some((start, _end)) = cached_match {
                if start >= self.pos {
                    // Can reuse cached match - need to find it again
                    pattern.find_at(&self.text, cached_pos)
                } else {
                    // Need new search
                    pattern.find_at(&self.text, self.pos)
                }
            } else {
                // No match was found previously
                None
            }
        } else {
            // No cache, do fresh search
            pattern.find_at(&self.text, self.pos)
        };

        // Update cache and convert to MatchInfo
        let result = match_result.map(|m| {
            let start = m.start();
            let end = m.end();
            let text = m.as_str().to_string();
            self.past_matches
                .insert(pattern_key.to_string(), (self.pos, Some((start, end))));
            MatchInfo { start, end, text }
        });

        if result.is_none() {
            self.past_matches
                .insert(pattern_key.to_string(), (self.pos, None));
        }

        result
    }

    fn first_match(&mut self, patterns: &[(&Regex, &str)]) -> Option<MatchInfo> {
        let mut matches = Vec::new();

        for (pattern, key) in patterns {
            if let Some(m) = self.search(pattern, key) {
                matches.push(m);
            }
        }

        matches.into_iter().min_by_key(|m| m.end)
    }

    fn expect_match(
        &mut self,
        expected_name: &str,
        patterns: &[(&Regex, &str)],
    ) -> Result<MatchInfo, TagIteratorError> {
        let pos = self.pos;
        self.first_match(patterns).ok_or_else(|| {
            let remaining = if pos < self.text.len() {
                self.text[pos..].to_string()
            } else {
                String::new()
            };
            TagIteratorError::UnexpectedEOF(expected_name.to_string(), remaining)
        })
    }

    pub fn handle_expr(&mut self, match_info: MatchInfo) -> Result<(), TagIteratorError> {
        self.advance(match_info.end);

        loop {
            let m = self.expect_match(
                "}}",
                &[
                    (&EXPR_END_PATTERN, "expr_end"),
                    (&QUOTE_START_PATTERN, "quote_start"),
                ],
            )?;

            if EXPR_END_PATTERN.is_match(&m.text) {
                self.advance(m.end);
                break;
            } else {
                // It's a quote, handle the string
                let string_match = self.expect_match("string", &[(&STRING_PATTERN, "string")])?;
                self.advance(string_match.end);
            }
        }

        Ok(())
    }

    pub fn handle_comment(&mut self, match_info: MatchInfo) -> Result<(), TagIteratorError> {
        self.advance(match_info.end);
        let m = self.expect_match("#}", &[(&COMMENT_END_PATTERN, "comment_end")])?;
        self.advance(m.end);
        Ok(())
    }

    fn expect_block_close(&mut self) -> Result<(), TagIteratorError> {
        loop {
            let end_match = self.expect_match(
                "tag close (\"%}\")",
                &[
                    (&QUOTE_START_PATTERN, "quote_start"),
                    (&TAG_CLOSE_PATTERN, "tag_close"),
                ],
            )?;

            if TAG_CLOSE_PATTERN.is_match(&end_match.text) {
                self.advance(end_match.end);
                return Ok(());
            }

            // Must be a string, don't advance yet, just handle the string
            let string_match = self.expect_match("string", &[(&STRING_PATTERN, "string")])?;
            self.advance(string_match.end);
        }
    }

    pub fn handle_raw(&mut self) -> Result<usize, TagIteratorError> {
        let m = self.expect_match("{% raw %}...{% endraw %}", &[(&RAW_BLOCK_PATTERN, "raw")])?;
        let end = m.end;
        self.advance(end);
        Ok(end)
    }

    pub fn handle_tag(&mut self, match_info: MatchInfo) -> Result<Tag, TagIteratorError> {
        let captures = BLOCK_START_PATTERN
            .captures(&match_info.text)
            .ok_or_else(|| {
                TagIteratorError::Internal("Failed to capture block start".to_string())
            })?;

        let block_type_name = captures
            .name("block_type_name")
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        let block_name = captures.name("block_name").map(|m| m.as_str().to_string());

        let start_pos = self.pos;

        if block_type_name == "raw" {
            let m =
                self.expect_match("{% raw %}...{% endraw %}", &[(&RAW_BLOCK_PATTERN, "raw")])?;
            self.advance(m.end);
        } else {
            self.advance(match_info.end);
            self.expect_block_close()?;
        }

        Ok(Tag {
            block_type_name,
            block_name,
            start: start_pos,
            end: self.pos,
        })
    }

    pub fn find_tags(&mut self) -> Vec<Tag> {
        let mut tags = Vec::new();

        loop {
            let m = self.first_match(&[
                (&BLOCK_START_PATTERN, "block_start"),
                (&COMMENT_START_PATTERN, "comment_start"),
                (&EXPR_START_PATTERN, "expr_start"),
            ]);

            let Some(match_info) = m else {
                break;
            };

            self.advance(match_info.start);

            if COMMENT_START_PATTERN.is_match(&match_info.text) {
                let _ = self.handle_comment(match_info);
            } else if EXPR_START_PATTERN.is_match(&match_info.text) {
                let _ = self.handle_expr(match_info);
            } else if BLOCK_START_PATTERN.is_match(&match_info.text) {
                if let Ok(tag) = self.handle_tag(match_info) {
                    tags.push(tag);
                }
            }
        }

        tags
    }
}

impl IntoIterator for TagIterator {
    type Item = Tag;
    type IntoIter = std::vec::IntoIter<Tag>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.find_tags().into_iter()
    }
}
