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
    let clipboard_contents = ClipboardContext::new().unwrap().get_contents().unwrap();

    let mut result: Vec<Vec<String>> = vec![];

    for row in clipboard_contents.lines() {
        if row.trim() == "" {
            continue;
        }

        let parse_row: Vec<String> = row
            .split("\t")
            .map(|s| s.to_string().replace("\"", "\\\"").replace("`", "\\`"))
            .collect();

        result.push(parse_row);
    }

    let headers = result[0].clone();
    result.remove(0);

    return (headers, result);
}
