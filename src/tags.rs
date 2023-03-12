use crate::prelude::*;

static mut TAGS: Vec<String> = Vec::new();

/// initialize tag index `TAGS` from a list of jobs
pub fn init(jobs: &Jobs) {
    unsafe {
        TAGS.clear();
        for job in &jobs.jobs {
            for tag in &job.tags.0 {
                TAGS.push(tag.clone());
            }
        }
    }
}

/// decorate tag with color
pub fn format(f: &mut std::fmt::Formatter, tag: &String) -> std::fmt::Result {
    use termion::{
        color::{Bg, Fg, *},
        style,
    };
    write!(f, "{}", style::Bold)?;
    if let Some(position) = position(&tag) {
        match position % 11 {
            0 => write!(f, "{}{} {} ", Bg(LightCyan), Fg(Black), &tag,)?,
            1 => write!(f, "{}{} {} ", Bg(LightMagenta), Fg(Black), &tag,)?,
            2 => write!(f, "{}{} {} ", Bg(LightYellow), Fg(Black), &tag,)?,
            3 => write!(f, "{}{} {} ", Bg(LightBlue), Fg(Black), &tag,)?,
            4 => write!(f, "{}{} {} ", Bg(LightGreen), Fg(Black), &tag,)?,
            5 => write!(f, "{}{} {} ", Bg(White), Fg(Black), &tag,)?,
            6 => write!(f, "{}{} {} ", Bg(Cyan), Fg(Black), &tag,)?,
            7 => write!(f, "{}{} {} ", Bg(Magenta), Fg(Black), &tag,)?,
            8 => write!(f, "{}{} {} ", Bg(Yellow), Fg(Black), &tag,)?,
            9 => write!(f, "{}{} {} ", Bg(Blue), Fg(Black), &tag,)?,
            10 => write!(f, "{}{} {} ", Bg(Green), Fg(Black), &tag,)?,
            _ => panic!("tag index error"),
        }
    } else {
        write!(f, "{}{} {} ", Bg(Red), Fg(White), &tag,)?;
    }
    write!(f, "{}{}{}", style::Reset, Fg(Reset), Bg(Reset))
}

/// get the position of a tag within the tag index `TAGS` (to assign a color)
fn position(tag: &String) -> Option<usize> {
    unsafe {
        if let Some(position) = TAGS.iter().position(|t| t == tag).into() {
            Some(position)
        } else {
            None
        }
    }
}

pub(crate) fn is_known(tag: &str) -> bool {
    unsafe { TAGS.contains(&tag.to_string()) }
}
