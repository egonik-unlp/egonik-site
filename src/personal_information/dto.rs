#[derive(Debug, Clone)]
pub struct PersonalInformationDto {
    id: i32,
    name: String,
    surname: String,
    image_url: String,
    // birth_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct ContactInformationDto {
    personal_information: PersonalInformationDto,
    github: String,
    email: String,
    instagram: String,
    linked_in: String,
}
