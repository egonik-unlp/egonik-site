use leptos::prelude::*;

use crate::personal_information::dto::{ContactInformationDto, PersonalInformationDto};

#[component]
pub fn WhoAmI(personal_information: PersonalInformationDto) -> impl IntoView {
    let PersonalInformationDto {
        id,
        name,
        surname,
        image_url,
        birth_date,
    } = personal_information;
    view! {
        <div>
                <p>{ format!("{} {}", name, surname)}</p>
                <p>{ format!("{} {}",  id, birth_date)}</p>
                <img src = image_url> </img>
        </div>
    }
}

#[component]
pub fn WhoAmIContact(
    personal_information: PersonalInformationDto,
    contact_information: ContactInformationDto,
) -> impl IntoView {
    let PersonalInformationDto {
        id,
        name,
        surname,
        image_url,
        birth_date,
    } = personal_information;
    let ContactInformationDto {
        github,
        email,
        instagram,
        linked_in,
    } = contact_information;
    view! {
        <div>
        <div>
                <p>{ format!("{} {}", name, surname)}</p>
                <p>{ format!("{} {}",  id, birth_date)}</p>
                <img src = image_url> </img>
        </div>

        <div>
                <p>{ format!("{} {}", github, email)}</p>
                <p>{ format!("{} {}",  instagram, email)}</p>
                <p>{ format!(" {} ",  linked_in)}</p>
        </div>
        </div>
    }
}
