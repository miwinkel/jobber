use crate::{configuration::Configuration, job::Job, tag_set::TagSet};

/// catches what to change the jobs within the database
#[derive(Clone, Debug)]
pub enum Change {
    /// No change
    Nothing,
    /// Push a new `Job` into database
    Push(Job),
    /// Change an existing `Job` at index `usize` into database but return error if message is missing
    Modify(usize, Job),
    /// Imported `usize` entries
    Import(usize, TagSet),
    /// Change configuration
    Configuration(Option<Vec<String>>, Configuration),
}

impl std::fmt::Display for Change {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nothing => {
                write!(f, "Database unchanged.")
            }
            Self::Push(job) => {
                if job.is_open() {
                    write!(f, "Started new job:\n\n{job}")
                } else {
                    write!(f, "Added new job:\n\n{job}")
                }
            }
            Self::Modify(position, job) => {
                if job.is_open() {
                    write!(f, "Ended open job:\n\n    Pos: {}\n{job}", position + 1)
                } else {
                    write!(f, "Modified job:\n\n    Pos: {}\n{job}", position + 1)
                }
            }
            Self::Import(count, new_tags) => {
                if new_tags.is_empty() {
                    write!(f, "Imported {count} jobs.")
                } else {
                    write!(f, "Imported {count} jobs added new tags {new_tags}.")
                }
            }
            Self::Configuration(tags, config) => {
                if let Some(tags) = tags {
                    write!(
                        f,
                        "Changed the following configuration values for tag(s) {}:\n\n{}",
                        TagSet { 0: tags.clone() },
                        config
                    )
                } else {
                    write!(
                        f,
                        "Changed the following default configuration values:\n\n{}",
                        config
                    )
                }
            }
        }
    }
}