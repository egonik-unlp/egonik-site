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
        <div class="card flex items-center gap-4">
            <img class="size-20 shrink-0 rounded-full object-cover" alt="" src=image_url />
            <div class="min-w-0">
                <p class="text-lg font-medium">{format!("{} {}", name, surname)}</p>
                <p class="muted">{format!("{} {}", id, birth_date)}</p>
            </div>
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
        <div class="grid gap-4 sm:grid-cols-2">
            <div class="card flex items-center gap-4">
                <img class="size-20 shrink-0 rounded-full object-cover" alt="" src=image_url />
                <div class="min-w-0">
                    <p class="text-lg font-medium">{format!("{} {}", name, surname)}</p>
                    <p class="muted">{format!("{} {}", id, birth_date)}</p>
                </div>
            </div>

            <div class="card space-y-1 text-sm">
                <p class="break-all">{format!("{} {}", github, email)}</p>
                <p class="break-all">{format!("{} {}", instagram, email)}</p>
                <p class="break-all">{format!(" {} ", linked_in)}</p>
            </div>
        </div>
    }
}
