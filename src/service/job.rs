use super::super::{
    domain,
    repository,
};

pub fn index(page: i32, page_size: i32) -> Vec<domain::job::DomainJob> {
    let repository_job = repository::job::RepositoryJob::new();
    let domain_jobs = repository_job.find_jobs(page, page_size);
    domain_jobs
}

pub fn show(job_id: i32) -> domain::job::DomainJob {
    let repository_job = repository::job::RepositoryJob::new();
    let domain_job = repository_job.find_job(job_id);
    domain_job
}