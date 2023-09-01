use colored::Colorize;
use inquire::{
    ui::{Color, RenderConfig, StyleSheet, Styled},
    Password, PasswordDisplayMode, Select, Text,
};

#[cfg(target_os = "windows")]
pub fn clear_screen() {
    std::process::Command::new("cmd")
        .args(&["/C", "cls"])
        .status()
        .expect("Failed to clear screen");
}

#[cfg(not(target_os = "windows"))]
pub fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}

fn themecfg() -> RenderConfig {
    let mut default_cfg = RenderConfig::default();

    default_cfg.selected_option = Some(StyleSheet::default().with_fg(Color::LightGreen));
    default_cfg.answer = StyleSheet::default().with_fg(Color::LightCyan);
    default_cfg.highlighted_option_prefix = Styled::new("➤").with_fg(Color::LightCyan);
    default_cfg.answered_prompt_prefix = Styled::new("➤").with_fg(Color::LightGreen);

    default_cfg
}

pub fn select(options: Vec<&str>, message: &str) -> Option<String> {
    let mut select_mode = Select::new(message, options);
    select_mode.vim_mode = false;
    select_mode.help_message = None;
    select_mode.render_config = themecfg();

    let ans = match select_mode.prompt() {
        Ok(ans) => Some(ans.to_string().to_lowercase()),
        Err(_) => return None,
    };

    ans
}

pub fn ask_password(message: &str, enable_confirmation: bool) -> Option<String> {
    let mut password_mode = Password::new(message);

    password_mode.help_message = Some("Ctrl + r for show password");
    password_mode.display_mode = PasswordDisplayMode::Masked;
    password_mode.enable_display_toggle = true;
    password_mode.enable_confirmation = enable_confirmation;

    let ans = match password_mode.prompt() {
        Ok(r) => r,
        Err(_) => return None,
    };

    Some(ans)
}

pub fn ask(message: &str) -> Option<String> {
    let mut ask_mode = Text::new(message);

    ask_mode.render_config = themecfg();

    let ans = match ask_mode.prompt() {
        Ok(r) => r,
        Err(_) => return None,
    };

    Some(ans)
}

pub fn tree_classic(title: &str, elements: Vec<&str>) {
    let spaces = " ".repeat(3);
    let elen = elements.len() - 1;

    println!("\n{spaces}{}\n{spaces}|", title);
    for (i, e) in elements.iter().enumerate() {
        if elen <= i {
            println!("{spaces}└── [ {} ]\n", e.bright_green());
        } else {
            println!("{spaces}├── [ {} ]\n{spaces}|", e.bright_green());
        }
    }
}

pub fn table(headers: Vec<&str>, rows: Vec<Vec<String>>, style_c: char, spaced: usize) {
    // Calculates the maximum width for each column
    let mut max_widths = headers
        .iter()
        .map(|header| header.len())
        .collect::<Vec<_>>();

    for row in &rows {
        if row.len() != headers.len() {
            panic!("row_limit_exceeded");
        }
        for (i, cell) in row.iter().enumerate() {
            max_widths[i] = max_widths[i].max(cell.len());
        }
    }

    let tab_spacing = " ".repeat(spaced);

    println!();

    // Printing headers and separators
    println!(
        "{}",
        headers
            .iter()
            .enumerate()
            .map(|(i, header)| format!("{0:<width$}", header, width = max_widths[i]))
            .collect::<Vec<_>>()
            .join(&tab_spacing)
    );
    println!(
        "{}",
        max_widths
            .iter()
            .map(|width| style_c.to_string().repeat(*width))
            .collect::<Vec<_>>()
            .join(&tab_spacing)
    );

    // Printing rows
    for row in &rows {
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(i, cell)| format!("{0:<width$}", cell, width = max_widths[i]))
                .collect::<Vec<_>>()
                .join(&tab_spacing)
        );
    }

    println!();
}
