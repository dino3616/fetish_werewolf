use poise::serenity_prelude as serenity;

use super::Job;

#[derive(Debug)]
pub struct Attender {
    pub id: serenity::UserId,
    pub name: String,
    pub fetish: String,
    pub is_alive: bool,
    pub job: Job,
}

impl Attender {
    pub fn new(id: serenity::UserId, name: String, fetish: String) -> Self {
        Self {
            id,
            name,
            fetish,
            is_alive: true,
            job: Job::Villager,
        }
    }
}
