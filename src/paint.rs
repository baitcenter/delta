use std::fmt::Write;

use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, Theme};
use syntect::parsing::{SyntaxReference, SyntaxSet};

pub const DARK_THEMES: [&str; 4] = [
    "Solarized (dark)",
    "base16-eighties.dark",
    "base16-mocha.dark",
    "base16-ocean.dark",
];

const LIGHT_THEME_PLUS_COLOR: Color = Color {
    r: 0xd0,
    g: 0xff,
    b: 0xd0,
    a: 0xff,
};

const LIGHT_THEME_MINUS_COLOR: Color = Color {
    r: 0xff,
    g: 0xd0,
    b: 0xd0,
    a: 0xff,
};

const DARK_THEME_PLUS_COLOR: Color = Color {
    r: 0x01,
    g: 0x3B,
    b: 0x01,
    a: 0xff,
};

const DARK_THEME_MINUS_COLOR: Color = Color {
    r: 0x3F,
    g: 0x00,
    b: 0x01,
    a: 0xff,
};

pub struct Config<'a> {
    pub theme: &'a Theme,
    plus_color: Color,
    minus_color: Color,
    pub syntax_set: SyntaxSet,
    pub width: Option<usize>,
}

pub fn get_config<'a>(
    theme: &'a Theme,
    theme_name: &'a String,
    plus_color: Option<Color>,
    minus_color: Option<Color>,
    width: Option<usize>,
) -> Config<'a> {

    let is_dark = DARK_THEMES.contains(&theme_name.as_str());

    Config {
        theme: theme,
        plus_color: plus_color.unwrap_or_else(|| if is_dark {
            DARK_THEME_PLUS_COLOR
        } else {
            LIGHT_THEME_PLUS_COLOR
        }),
        minus_color: minus_color.unwrap_or_else(|| if is_dark {
            DARK_THEME_MINUS_COLOR
        } else {
            LIGHT_THEME_MINUS_COLOR
        }),
        width: width,
        syntax_set: SyntaxSet::load_defaults_newlines(),
    }
}

/// Write line to buffer with color escape codes applied.
pub fn paint_line(mut line: String, syntax: &SyntaxReference, config: &Config, buf: &mut String) {
    let mut highlighter = HighlightLines::new(syntax, config.theme);
    let first_char = line.chars().next();
    let background_color = match first_char {
        Some('+') => Some(config.plus_color),
        Some('-') => Some(config.minus_color),
        _ => None,
    };
    if first_char == Some('+') || first_char == Some('-') {
        line = line[1..].to_string();
        buf.push_str(" ");
    }
    match config.width {
        Some(width) => {
            if line.len() < width {
                line = format!("{}{}", line, " ".repeat(width - line.len()));
            }
        }
        _ => (),
    }
    let ranges: Vec<(Style, &str)> = highlighter.highlight(&line, &config.syntax_set);
    paint_ranges(&ranges[..], background_color, buf);
}

/// Based on as_24_bit_terminal_escaped from syntect
fn paint_ranges(
    foreground_style_ranges: &[(Style, &str)],
    background_color: Option<Color>,
    buf: &mut String,
) -> () {
    for &(ref style, text) in foreground_style_ranges.iter() {
        paint(text, Some(style.foreground), background_color, false, buf);
    }
    buf.push_str("\x1b[0m");
}

/// Write text to buffer with color escape codes applied.
fn paint(
    text: &str,
    foreground_color: Option<Color>,
    background_color: Option<Color>,
    reset_color: bool,
    buf: &mut String,
) -> () {
    match background_color {
        Some(background_color) => {
            write!(
                buf,
                "\x1b[48;2;{};{};{}m",
                background_color.r,
                background_color.g,
                background_color.b
            ).unwrap();
            if reset_color {
                buf.push_str("\x1b[0m");
            }
        }
        None => (),
    }
    match foreground_color {
        Some(foreground_color) => {
            write!(
                buf,
                "\x1b[38;2;{};{};{}m{}",
                foreground_color.r,
                foreground_color.g,
                foreground_color.b,
                text
            ).unwrap();
            if reset_color {
                buf.push_str("\x1b[0m");
            }
        }
        None => {
            write!(buf, "{}", text).unwrap();
        }
    }
}