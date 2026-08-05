use crate::{
    database::connection::DbPool,
    job_experience::repository::JobExperienceItemRepository,
    personal_information::{
        repository::PersonalInformationRepository, service::PersonalInformationService,
    },
    portfolio::{repository::PortfolioItemsRepository, service::PortfolioService},
    publications::{repository::PublicationsRepository, service::PublicationsService},
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub job_experience_repository: JobExperienceItemRepository,
    pub personal_information_service: PersonalInformationService,
    pub portfolio_service: PortfolioService,
    pub publications_service: PublicationsService,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        let personal_information_service =
            PersonalInformationService::new(PersonalInformationRepository::new(pool.clone()));
        let job_experience_repository = JobExperienceItemRepository::new(pool.clone());
        let portfolio_service = PortfolioService::new(PortfolioItemsRepository::new(pool.clone()));
        let publications_service = PublicationsService::new(PublicationsRepository::new(pool));

        Self {
            personal_information_service,
            job_experience_repository,
            portfolio_service,
            publications_service,
        }
    }
}
