use copypasta::{ClipboardContext, ClipboardProvider};
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};

pub fn cli_input(question: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .default("".into())
        .interact_text()
        .unwrap()
}

pub fn cli_select(items: Vec<String>) -> Option<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
}

pub fn parse_clipboard() -> (Vec<String>, Vec<Vec<String>>) {
    let mut ctx = ClipboardContext::new().unwrap();

    let clipboard_contents = ctx.get_contents().unwrap();

    let mut headers: Vec<String> = vec![];
    let mut result: Vec<Vec<String>> = vec![];
    let mut index = 0;

    for row in clipboard_contents.lines() {
        index += 1;

        if row.trim() == "" {
            continue;
        }

        let parse_row = row
            .split("\t")
            .map(|s| s.to_string().replace("\"", "\\\"").replace("`", "\\`"))
            .collect::<Vec<String>>();

        if index == 1 {
            headers = parse_row;
        } else {
            result.push(parse_row);
        }
    }

    (headers, result)
}
