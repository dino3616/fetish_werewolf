use super::Job;

#[derive(Debug)]
pub struct Attender {
    pub id: String,
    pub name: String,
    pub fetish: String,
    pub is_alive: bool,
    pub job: Job,
}
