use std::borrow::Cow::{self, Borrowed, Owned};

use rustyline::error::ReadlineError;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::validate::Validator;
use rustyline::validate::{ValidationContext, ValidationResult};
use rustyline::{CompletionType, Config, Context, EditMode};
use rustyline_derive::{Completer, Helper};

#[derive(Helper, Completer)]
pub struct REPLHelper {
    pub colored_prompt: String,
    pub hinter: HistoryHinter,
    pub highlighter: MatchingBracketHighlighter,
}

impl Default for REPLHelper {
    fn default() -> Self {
        Self {
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            colored_prompt: "".to_owned(),
        }
    }
}

impl Hinter for REPLHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Validator for REPLHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        use ValidationResult::{Incomplete, Valid};
        let input = ctx.input();
        let result = if input.starts_with(".") {
            Valid(None)
        } else if input.is_empty() {
            Valid(None)
        } else if !input.ends_with(';') {
            Incomplete
        } else {
            Valid(None)
        };
        Ok(result)
    }
}

impl Highlighter for REPLHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

pub fn get_config() -> Config {
    Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build()
}
