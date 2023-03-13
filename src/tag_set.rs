use super::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TagSet(pub Vec<String>);

impl TagSet {
    pub const fn new() -> Self {
        Self(Vec::new())
    }
    pub fn from_option_vec(tags: Option<Vec<String>>) -> Option<Self> {
        if let Some(tags) = tags {
            Some(tags.into())
        } else {
            None
        }
    }
    pub fn filter<P>(&self, pred: P) -> Self
    where
        P: Fn(&&String) -> bool,
    {
        TagSet(self.0.iter().filter(pred).map(|t| t.to_string()).collect())
    }
    pub fn iter(&self) -> core::slice::Iter<'_, String> {
        self.0.iter()
    }
    pub fn contains(&self, tag: &String) -> bool {
        self.0.contains(tag)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn modify(&self, modification: &TagSet) -> TagSet {
        let mut modify = false;
        let mut tags = TagSet::new();
        for tag in modification.iter() {
            if tag.starts_with('+') || tag.ends_with('+') {
                tags.insert(tag[1..].into());
                modify = true;
            } else if tag.starts_with('-') || tag.ends_with('-') {
                tags.remove(tag[1..].into());
                modify = true;
            }
        }
        if modify {
            tags
        } else {
            modification.clone()
        }
    }
}

impl TagSet {
    pub fn insert(&mut self, tag: &str) -> bool {
        if self.0.contains(&tag.to_string()) {
            false
        } else {
            self.0.push(tag.to_string());
            true
        }
    }
    pub fn insert_many(&mut self, tags: Vec<String>) {
        for tag in tags {
            self.insert(&tag);
        }
    }
    pub fn remove(&mut self, tag: &str) {
        self.0 = self
            .0
            .iter()
            .filter(|t| *t != tag)
            .map(|t| t.to_string())
            .collect();
    }
}
impl std::fmt::Display for TagSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n, tag) in self.0.iter().enumerate() {
            tags::format(f, tag)?;
            if n + 1 < self.0.len() {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}
impl From<Vec<String>> for TagSet {
    fn from(tags: Vec<String>) -> Self {
        let mut tags = tags.clone();
        tags.dedup();
        Self(tags)
    }
}
impl From<Option<Vec<String>>> for TagSet {
    fn from(tags: Option<Vec<String>>) -> Self {
        if let Some(tags) = tags {
            let mut tags = tags.clone();
            tags.dedup();
            Self(tags)
        } else {
            Self(Vec::new())
        }
    }
}
impl From<&Option<String>> for TagSet {
    fn from(tag: &Option<String>) -> Self {
        if let Some(tag) = tag {
            Self(tag.split('*').map(|t| t.to_string()).collect())
        } else {
            Self(Vec::new())
        }
    }
}
impl From<&String> for TagSet {
    fn from(tag: &String) -> Self {
        Self(tag.split('*').map(|t| t.to_string()).collect())
    }
}
impl From<&str> for TagSet {
    fn from(tag: &str) -> Self {
        Self(tag.split('*').map(|t| t.to_string()).collect())
    }
}
