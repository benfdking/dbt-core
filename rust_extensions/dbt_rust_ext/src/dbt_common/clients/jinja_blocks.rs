use regex::Regex;
use std::collections::HashMap;
use std::fmt;

pub struct BlockTag {
    pub block_type_name: String,
    pub block_name: String,
    pub contents: Option<String>,
    pub full_block: Option<String>,
}

const NAME_PATTERN: &str = r"[A-Za-z_][A-Za-z_0-9]*";

const COMMENT_START_PATTERN: &str = r"(?:(?P<comment_start>(\s*\{\#)))";
const COMMENT_END_PATTERN: &str = r"(.*?)(\s*\#\})";
const RAW_START_PATTERN: &str = r"(?:\s*\{\%\-|\{\%)\s*(?P<raw_start>(raw))\s*(?:\-\%\}\s*|\%\})";
const EXPR_START_PATTERN: &str = r"(?P<expr_start>(\{\{\s*))";
const EXPR_END_PATTERN: &str = r"(?P<expr_end>(\s*\}\}))";

pub fn comment_start_pattern() -> Regex {
    Regex::new(COMMENT_START_PATTERN).unwrap()
}
pub fn comment_end_pattern() -> Regex {
    Regex::new(COMMENT_END_PATTERN).unwrap()
}
pub fn raw_start_pattern() -> Regex {
    Regex::new(RAW_START_PATTERN).unwrap()
}
pub fn expr_start_pattern() -> Regex {
    Regex::new(EXPR_START_PATTERN).unwrap()
}
pub fn expr_end_pattern() -> Regex {
    Regex::new(EXPR_END_PATTERN).unwrap()
}

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
pub struct PositionedMatch {
    pub start_pos: usize,
    pub match_: Option<regex::Match<'static>>,
}

impl PositionedMatch {
    pub fn new(start_pos: usize, match_: Option<regex::Match<'static>>) -> Self {
        Self { start_pos, match_ }
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub block_type_name: String,
    pub block_name: Option<String>,
    pub start: usize,
    pub end: usize,
}

impl Tag {
    pub fn new(
        block_type_name: String,
        block_name: Option<String>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            block_type_name,
            block_name,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TagIterator {
    pub text: String,
    pub pos: usize,
    // A cache of the most recent matches seen for each pattern, maintained
    // in order to avoid slowly re-searching long inputs many times.
    pub past_matches: HashMap<String, (Regex, PositionedMatch)>,
}

impl TagIterator {
    pub fn new(text: String) -> Self {
        Self {
            text,
            pos: 0,
            past_matches: HashMap::new(),
        }
    }

    // Return relative position in line.
    // Given an absolute position in the input data, return a pair of
    // line number + relative position to the start of the line.
    pub fn linepos(&self, end: Option<isize>) -> String {
        let end_val = end.unwrap_or(self.pos as isize);
        let text = &self.text[..end_val as usize];
        // if not found, rfind returns None, so we use 0 as the start
        let last_line_start = text.rfind('\n').map(|idx| idx + 1).unwrap_or(0);
        // line numbers are 1-indexed
        let line_number = text.chars().filter(|&c| c == '\n').count() + 1;
        format!("{}:{}", line_number, end_val - last_line_start as isize)
    }

    pub fn advance(&mut self, new_position: usize) {
        self.pos = new_position;
    }

    pub fn rewind(&mut self, amount: usize) {
        self.pos = self.pos - amount;
    }

    fn _search(&mut self, pattern: &'static str) -> Option<PositionedMatch> {
        // Check to see if we have cached a search for this pattern already.
        let positioned_match = self.past_matches.get(pattern).cloned();

        if positioned_match.is_none() || positioned_match.as_ref().unwrap().1.start_pos > self.pos {
            // We did not have a cached search, or we did, but it was done at a location
            // further along in the string and can't be used. Do a search and cache it.
            let regex = Regex::new(pattern).unwrap();
            let match_result = regex.find_at(&self.text, self.pos as usize);
            let positioned_match = PositionedMatch::new(self.pos, match_result.clone());
            self.past_matches
                .insert(pattern.to_string(), (regex, positioned_match.clone()));
            Some(positioned_match)
        } else {
            let positioned_match = positioned_match.unwrap();
            // We have a cached search and its start position falls before (or at) the
            // current search position...
            if positioned_match.1.match_.is_none() {
                // ...but there is no match in the rest of the text.
                None
            } else if positioned_match.1.match_.as_ref().unwrap().start() >= self.pos {
                Some(PositionedMatch {
                    start_pos: self.pos,
                    match_: positioned_match.1.match_.clone(),
                })
            } else {
                // ...but we have passed the start of the cached match, and need to do a
                // new search from our current position and cache it.
                let (regex, _) = self.past_matches.get(pattern).unwrap();
                let match_result = regex.find_at(&self.text, self.pos as usize);
                let new_positioned_match =
                    PositionedMatch::new(self.pos.clone(), match_result.clone());
                self.past_matches.insert(
                    pattern.to_string(),
                    (regex.clone(), new_positioned_match.clone()),
                );
                Some(new_positioned_match)
            }
        }
    }

    fn _first_match(&mut self, patterns: &[&str]) -> Option<PositionedMatch> {
        let mut matches = Vec::new();
        for pattern in patterns {
            if let Some(matched) = self._search(pattern) {
                matches.push(matched);
            }
        }
        if matches.is_empty() {
            return None;
        }
        // if there are multiple matches, pick the least greedy match
        // TODO: do I need to account for match.start(), or is this ok?
        matches
            .into_iter()
            .min_by_key(|m| m.match_.as_ref().unwrap().end())
    }

    pub fn find_tags(&self) -> Vec<Tag> {
        loop {
            unimplemented!()
        }

        unimplemented!("find_tags not implemented")
    }
}

use std::option::Option;
use std::vec::Vec;

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

    /// Find all top-level blocks in the data.
    pub fn find_blocks(
        &self,
        allowed_blocks: Option<&std::collections::HashSet<String>>,
        collect_raw_data: Option<bool>,
    ) -> Vec<BlockTag> {
        let allowed_blocks: &std::collections::HashSet<String> =
            if let Some(blocks) = allowed_blocks {
                blocks
            } else {
                &["snapshot", "macro", "materialization", "docs"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<std::collections::HashSet<String>>()
            };

        for tag in &self.tag_parser.find_tags() {
            unimplemented!("find_blocks not implemented")
        }

        unimplemented!("find_blocks not implemented")
    }

    pub fn lex_for_blocks(
        &self,
        allowed_blocks: Option<&std::collections::HashSet<String>>,
        collect_raw_data: Option<bool>,
    ) -> Vec<BlockTag> {
        let collect_raw_data = collect_raw_data.unwrap_or(true);
        self.find_blocks(allowed_blocks, Some(collect_raw_data))
    }
}

pub static _CONTROL_FLOW_TAGS: [[&str; 2]; 2] = [["if", "endif"], ["for", "endfor"]];

pub fn _control_flow_tags() -> HashMap<String, String> {
    let mut tags = HashMap::new();
    for tag in _CONTROL_FLOW_TAGS {
        tags.insert(tag[0].to_string(), tag[1].to_string());
    }
    tags
}
