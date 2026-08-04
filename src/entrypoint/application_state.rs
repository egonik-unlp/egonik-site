use crate::{
    database::connection::DbPool, job_experience::repository::JobExperienceItemRepository,
    personal_information::repository::PersonalInformationRepository,
    portfolio::repository::PortfolioItemsRepository,
    publications::repository::PublicationsRepository,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub job_experience_repository: JobExperienceItemRepository,
    pub personal_information_repository: PersonalInformationRepository,
    pub portfolio_items_repository: PortfolioItemsRepository,
    pub publications_repository: PublicationsRepository,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        let personal_information_repository = PersonalInformationRepository::new(pool.clone());
        let job_experience_repository = JobExperienceItemRepository::new(pool.clone());
        let portfolio_items_repository = PortfolioItemsRepository::new(pool.clone());
        let publications_repository = PublicationsRepository::new(pool);
        Self {
            personal_information_repository,
            job_experience_repository,
            portfolio_items_repository,
            publications_repository,
        }
    }
}
