//! Formatting functions for job properties.

use super::prelude::*;
use separator::Separatable;

#[cfg(feature = "colors")]
use termion::{color::*, style};

/// Format start date and time with color.
#[cfg(feature = "colors")]
pub fn start(start: &DateTime) -> String {
    format!("{}{}{}", Fg(Green), start, Fg(Reset))
}

/// Format end date and time with color.
#[cfg(feature = "colors")]
pub fn end(end: &DateTime) -> String {
    format!("{}{}{}", Fg(Magenta), end, Fg(Reset))
}

/// return colored hours bar in a string
#[cfg(feature = "colors")]
pub fn hours_bar(hours: f64, properties: &Properties) -> String {
    fn bar(hours: f64) -> String {
        if hours > 0.0 && hours < 24.0 {
            " ".to_string()
                + &"+".repeat(hours as usize)
                + if hours.fract() > 0.5 {
                    "+"
                } else if hours.fract() > 0.25 {
                    "-"
                } else {
                    ""
                }
        } else {
            "".into()
        }
    }
    if let Some(max_hours) = properties.max_hours {
        if hours > max_hours as f64 {
            return format!(
                "{}{}{}{}{}",
                style::Bold,
                Fg(LightRed),
                bar(hours),
                Fg(Reset),
                style::Reset
            );
        }
    }
    format!(
        "{}{}{}{}{}",
        style::Bold,
        Fg(Yellow),
        bar(hours),
        Fg(Reset),
        style::Reset
    )
}

/// Format hours (considering resolution) with style & color.
#[cfg(feature = "colors")]
pub fn hours(hours: f64, properties: &Properties) -> String {
    if let Some(max_hours) = properties.max_hours {
        if hours > max_hours as f64 {
            return format!(
                "{}{}{}{}{}",
                style::Bold,
                Fg(LightYellow),
                hours,
                Fg(Reset),
                style::Reset,
            );
        }
    }
    hours_pure(hours)
}

/// Format exact hours with style & color.
#[cfg(feature = "colors")]
pub fn hours_pure(hours: f64) -> String {
    if hours > 24_f64 {
        format!(
            "{}{}{}{}{}",
            style::Bold,
            Fg(LightRed),
            hours,
            Fg(Reset),
            style::Reset,
        )
    } else {
        format!(
            "{}{}{}{}{}",
            style::Bold,
            Fg(White),
            hours,
            Fg(Reset),
            style::Reset
        )
    }
}

/// Format payment (considering resolution) with style & color.
pub fn pay(hours: f64, configuration: &Properties) -> String {
    if let Some(rate) = configuration.rate {
        return pay_pure(rate * hours);
    }
    String::new()
}

/// Format exact payment with style & color.
#[cfg(feature = "colors")]
pub fn pay_pure(pay: f64) -> String {
    format!(
        "{}{}{}{}{}",
        style::Bold,
        Fg(White),
        pay.separated_string(),
        style::Reset,
        Fg(Reset)
    )
}

/// Format message with style.
#[cfg(feature = "colors")]
pub fn message(message: &str, indent: usize) -> String {
    let mut output = String::new();
    let lines = message.split('\n');
    for line in lines {
        if output.is_empty() {
            output += &format!("{}{}{}", style::Bold, Fg(LightWhite), line);
        } else {
            output += "\n";
            for _ in 0..indent {
                output += " ";
            }
            output += line;
        }
    }
    output + &format!("{}{}", Fg(Reset), style::Reset)
}


/// Format start date and time with color.
#[cfg(not(feature = "colors"))]
pub fn start(start: &DateTime) -> String {
    format!("{}", start)
}

/// Format end date and time with color.
#[cfg(not(feature = "colors"))]
pub fn end(end: &DateTime) -> String {
    format!("{}", end)
}

/// return colored hours bar in a string
#[cfg(not(feature = "colors"))]
pub fn hours_bar(hours: f64, properties: &Properties) -> String {
    fn bar(hours: f64) -> String {
        if hours > 0.0 && hours < 24.0 {
            " ".to_string()
                + &"+".repeat(hours as usize)
                + if hours.fract() > 0.5 {
                    "+"
                } else if hours.fract() > 0.25 {
                    "-"
                } else {
                    ""
                }
        } else {
            "".into()
        }
    }
    if let Some(max_hours) = properties.max_hours {
        if hours > max_hours as f64 {
            return format!("{}", bar(hours));
        }
    }
    format!("{}", bar(hours))
}

/// Format hours (considering resolution) with style & color.
#[cfg(not(feature = "colors"))]
pub fn hours(hours: f64, properties: &Properties) -> String {
    if let Some(max_hours) = properties.max_hours {
        if hours > max_hours as f64 {
            return format!("{}", hours);
        }
    }
    hours_pure(hours)
}

/// Format exact hours with style & color.
#[cfg(not(feature = "colors"))]
pub fn hours_pure(hours: f64) -> String {
    format!("{}", hours)
}

/// Format exact payment with style & color.
#[cfg(not(feature = "colors"))]
pub fn pay_pure(pay: f64) -> String {
    format!("{}", pay.separated_string())
}

/// Format message with style.
#[cfg(not(feature = "colors"))]
pub fn message(message: &str, indent: usize) -> String {
    let mut output = String::new();
    let lines = message.split('\n');
    for line in lines {
        if output.is_empty() {
            output += &format!("{}", line);
        } else {
            output += "\n";
            for _ in 0..indent {
                output += " ";
            }
            output += line;
        }
    }
    output
}

