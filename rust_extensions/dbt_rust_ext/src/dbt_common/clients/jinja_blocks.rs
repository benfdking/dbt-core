use std::fmt;

pub struct BlockTag {
    pub block_type_name: String,
    pub block_name: String,
    pub contents: Option<String>,
    pub full_block: Option<String>,
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

use regex::Regex;
use std::collections::HashMap;

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
    pub pos: isize,
    // A cache of the most recent matches seen for each pattern, maintained
    // in order to avoid slowly re-searching long inputs many times.
    pub past_matches: HashMap<Regex, PositionedMatch>,
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
        let end_val = end.unwrap_or(self.pos);
        let text = &self.text[..end_val as usize];
        // if not found, rfind returns None, so we use 0 as the start
        let last_line_start = text.rfind('\n').map(|idx| idx + 1).unwrap_or(0);
        // line numbers are 1-indexed
        let line_number = text.chars().filter(|&c| c == '\n').count() + 1;
        format!("{}:{}", line_number, end_val - last_line_start as isize)
    }

    pub fn advance(&mut self, new_position: isize) {
        self.pos = new_position;
    }

    pub fn rewind(&mut self, amount: isize) {
        self.pos = self.pos - amount;
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

    pub fn find_blocks(
        &self,
        allowed_blocks: Option<&std::collections::HashSet<String>>,
        collect_raw_data: Option<bool>,
    ) -> Vec<BlockTag> {
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
