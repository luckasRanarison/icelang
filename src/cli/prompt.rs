use reedline::Prompt;
use std::borrow::Cow;

pub struct IcePrompt {}

impl IcePrompt {
    pub fn new() -> Self {
        Self {}
    }
}

impl Prompt for IcePrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(
        &self,
        _prompt_mode: reedline::PromptEditMode,
    ) -> std::borrow::Cow<str> {
        Cow::Borrowed("> ")
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        Cow::Borrowed(".. ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        _history_search: reedline::PromptHistorySearch,
    ) -> Cow<str> {
        Cow::Borrowed("")
    }
}
