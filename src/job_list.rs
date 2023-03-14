//! An indexed list of jobs which have been extracted from the [Jobs] database

use super::prelude::*;

/// Adds an index to a [Job] which stores the original position within the database.
type IndexedJob = (usize, Job);

/// List of jobs extracted from database.
#[derive(Debug, Clone)]
pub struct JobList {
    /// List of jobs (including original index within database).
    jobs: Vec<IndexedJob>,
    /// Copy of the configuration of the original [Jobs] database.
    pub configuration: Configuration,
}

impl IntoIterator for JobList {
    type Item = IndexedJob;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.jobs.into_iter()
    }
}

impl std::fmt::Display for JobList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pos, job) in self.iter() {
            writeln!(f, "    Pos: {}", pos + 1)?;
            job.writeln(f, self.configuration.get(&job.tags))?;
            writeln!(f, "")?;
        }
        let pay = {
            if let Some(pay) = self.pay_overall() {
                format!(" = ${}", format_pay_pure(pay))
            } else {
                String::new()
            }
        };
        writeln!(
            f,
            "Total: {} job(s), {} hours{}",
            self.len(),
            format_hours_pure(self.hours_overall()),
            pay,
        )?;
        Ok(())
    }
}

impl JobList {
    /// Create job list on base of the given database but does not copy the jobs themselves (but it's configuration).
    pub fn new_from(jobs: &Jobs) -> Self {
        Self {
            jobs: Vec::new(),
            configuration: jobs.configuration.clone(),
        }
    }
    /// Add new job.
    pub fn push(&mut self, pos: usize, job: Job) {
        self.jobs.push((pos, job))
    }
    /// Get iterator over included jobs.
    pub fn iter(&self) -> core::slice::Iter<'_, IndexedJob> {
        self.jobs.iter()
    }
    /// Return true if list is empty.
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
    /// Return the length of the list.
    pub fn len(&self) -> usize {
        self.jobs.len()
    }
    /// Take
    pub fn drain(&mut self, count: usize) -> Result<(), Error> {
        if count > self.jobs.len() {
            return Err(Error::ToFewJobs(count, self.jobs.len()));
        }
        self.jobs.drain(0..(self.jobs.len() - count));
        Ok(())
    }
    pub fn tags(&self) -> TagSet {
        let mut tags = TagSet::new();
        for (_, job) in &self.jobs {
            tags.insert_many(job.tags.0.clone());
        }
        tags
    }
    pub fn positions(&self) -> Positions {
        Positions::from_iter(self.jobs.iter().map(|(n, _)| *n))
    }
    /// provides configurations for display trait implementation
    pub fn get_configuration(&self, tags: &TagSet) -> &Properties {
        self.get_configuration_with_tag(tags).1
    }
    /// provides configurations for display trait implementation
    pub fn get_configuration_with_tag(&self, tags: &TagSet) -> (String, &Properties) {
        for tag in &tags.0 {
            if let Some(properties) = self.configuration.tags.get(tag) {
                return (tag.clone(), properties);
            }
        }
        (String::new(), &self.configuration.base)
    }
    pub fn hours_overall(&self) -> f64 {
        let mut hours = 0.0;
        for (_, job) in &self.jobs {
            hours += job.hours(self.get_configuration(&job.tags))
        }
        hours
    }
    pub fn pay_overall(&self) -> Option<f64> {
        let mut pay_sum = 0.0;
        let mut has_payment = false;
        for (_, job) in &self.jobs {
            let properties = self.get_configuration(&job.tags);
            if let Some(rate) = properties.rate {
                pay_sum += rate * job.hours(properties);
                has_payment = true;
            }
        }
        if has_payment {
            Some(pay_sum)
        } else {
            None
        }
    }
}
