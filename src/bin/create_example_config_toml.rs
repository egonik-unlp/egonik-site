use std::{fs::OpenOptions, io::Write, str::FromStr};

use anyhow::Context;
use chrono::NaiveDate;
use egonik_site::{
    core::config::model::InformationConfigFile,
    personal_information::model::{ContactInformationRow, PersonalInformationRow},
};

fn main() -> anyhow::Result<()> {
    let contact_information = ContactInformationRow::new(
        1,
        "egonik-unlp".to_string(),
        "eduardogonik@gmail.com".to_string(),
        "edugonik".to_string(),
        "edugonik".to_string(),
    );
    let personal_information = PersonalInformationRow::new(
        1,
        "Eduardo".to_string(),
        "Gonik".to_string(),
        "url imagen".to_string(),
        NaiveDate::from_str("1994-01-23").unwrap(),
    );
    let file = InformationConfigFile::new(personal_information, contact_information);
    let body = toml::to_string(&file).unwrap();
    let mut outfile = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("example-config.toml")
        .unwrap();
    outfile
        .write_all(body.as_bytes())
        .context("Can't write outfile")?;
    Ok(())
}
