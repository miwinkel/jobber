use crate::prelude::*;
use std::collections::HashMap;

/// list of jobs extracted from database
#[derive(Debug)]
pub struct JobList {
    /// list of jobs (including original index in database)
    jobs: Vec<(usize, Job)>,
    /// Configuration by tag
    pub tag_configuration: HashMap<String, Configuration>,
    /// Configuration used when no tag related configuration fit
    pub default_configuration: Configuration,
}

impl std::fmt::Display for JobList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pos, job) in &self.jobs {
            writeln!(f, "    Pos: {}", pos + 1)?;
            job.writeln(f, self.get_configuration(&job.tags))?;
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
            self.jobs.len(),
            format_hours_pure(self.hours_overall()),
            pay,
        )?;
        Ok(())
    }
}

impl JobList {
    /// create job list on base of the given database
    pub fn new_from(jobs: &Jobs) -> Self {
        Self {
            jobs: Vec::new(),
            tag_configuration: jobs.tag_configurations.clone(),
            default_configuration: jobs.base_configuration.clone(),
        }
    }
    /// add new job
    pub fn push(&mut self, pos: usize, job: Job) {
        self.jobs.push((pos, job))
    }
    /// returns true if list is empty
    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }
    pub fn len(&self) -> usize {
        self.jobs.len()
    }
    pub fn limit(&mut self, count: usize) {
        while self.jobs.len() > count {
            self.jobs.remove(0);
        }
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
    pub fn get_configuration(&self, tags: &TagSet) -> &Configuration {
        self.get_configuration_with_tag(tags).1
    }
    /// provides configurations for display trait implementation
    pub fn get_configuration_with_tag(&self, tags: &TagSet) -> (String, &Configuration) {
        for tag in &tags.0 {
            if let Some(configuration) = self.tag_configuration.get(tag) {
                return (tag.clone(), configuration);
            }
        }
        (String::new(), &self.default_configuration)
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
            let configuration = self.get_configuration(&job.tags);
            if let Some(pay) = configuration.pay {
                pay_sum += pay * job.hours(configuration);
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

pub struct JobListIterator<'a> {
    jobs: &'a JobList,
    index: usize,
}

impl<'a> Iterator for JobListIterator<'a> {
    type Item = (usize, Job);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.jobs.len() {
            let result = Some(self.jobs.jobs[self.index].clone());
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a JobList {
    type Item = (usize, Job);
    type IntoIter = JobListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            jobs: self,
            index: 0,
        }
    }
}
