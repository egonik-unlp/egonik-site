use crate::publications::metadata::{PublicationMetadataDto, PublicationMetadataTableDto};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicationItemDto {
    pub title: String,
    pub abs: String,
    pub year: i32,
    pub journal: String,
    pub link: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicationItemWithMetadataDto {
    pub publication: PublicationItemDto,
    pub metadata: PublicationMetadataDto,
}

impl PublicationItemWithMetadataDto {
    pub fn new(publication: PublicationItemDto, metadata: PublicationMetadataDto) -> Self {
        Self {
            publication,
            metadata,
        }
    }
}

impl PublicationItemDto {
    pub fn new(title: String, abs: String, year: i32, journal: String, link: String) -> Self {
        Self {
            title,
            abs,
            year,
            journal,
            link,
        }
    }
    /// Titles come from two independent sources — OpenAlex (which passes
    /// Crossref's markup straight through) and the curated metadata file — so
    /// they are only ever compared normalised.
    pub fn is_its_metadata(&self, other: &PublicationMetadataDto) -> bool {
        let Some(theirs) = other.title.as_deref().map(normalize_title) else {
            return false;
        };
        let ours = normalize_title(&self.title);
        if ours == theirs {
            return true;
        }
        // The two sources disagree on whether a trailing subtitle belongs to
        // the title, so a long enough shared prefix also counts as the same work.
        let (short, long) = if ours.len() < theirs.len() {
            (ours, theirs)
        } else {
            (theirs, ours)
        };
        short.len() >= MIN_PREFIX_MATCH && long.starts_with(&short)
    }
}

/// Shortest shared prefix accepted as a match: long enough that two distinct
/// papers cannot collide, short enough to absorb a dropped subtitle.
const MIN_PREFIX_MATCH: usize = 30;

/// Reduces a title to comparable form: drops the HTML Crossref embeds
/// (`<scp>`, `<i>`, …), drops case and punctuation, and collapses whitespace —
/// so `Zr( <scp>iv</scp> )` and `Zr(IV)` normalise to the same `zr iv`.
fn normalize_title(title: &str) -> String {
    let mut normalized = String::with_capacity(title.len());
    let mut inside_tag = false;
    let mut pending_separator = false;

    for character in title.chars() {
        match character {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if inside_tag => {}
            _ if character.is_alphanumeric() => {
                if pending_separator && !normalized.is_empty() {
                    normalized.push(' ');
                }
                pending_separator = false;
                normalized.extend(character.to_lowercase());
            }
            // Punctuation and whitespace alike collapse into a single space.
            _ => pending_separator = true,
        }
    }

    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    fn publication(title: &str) -> PublicationItemDto {
        PublicationItemDto::new(
            title.into(),
            String::new(),
            2022,
            String::new(),
            String::new(),
        )
    }

    fn metadata(title: &str) -> PublicationMetadataDto {
        PublicationMetadataDto {
            title: Some(title.into()),
            ..Default::default()
        }
    }

    #[test]
    fn matches_through_crossref_markup() {
        assert!(publication(
            "Development of hybrid nanoparticles based on Zr( <scp>iv</scp> ) and \
             perylene-3,4,9,10-tetracarboxylic acid with visible-light photoredox activity"
        )
        .is_its_metadata(&metadata(
            "Development of hybrid nanoparticles based on Zr(IV) and \
             perylene-3,4,9,10-tetracarboxylic acid with visible-light photoredox activity"
        )));

        assert!(publication(
            "<i>Staphylococcus aureus</i> biofilm eradication by the synergistic effect"
        )
        .is_its_metadata(&metadata(
            "Staphylococcus aureus biofilm eradication by the synergistic effect"
        )));
    }

    #[test]
    fn matches_when_one_source_drops_the_subtitle() {
        assert!(publication(
            "Environmentally Induced Changes of Commercial Carbon Nanotubes in Aqueous \
             Suspensions. Adaptive Behavior of Bacteria in Biofilms"
        )
        .is_its_metadata(&metadata(
            "Environmentally Induced Changes of Commercial Carbon Nanotubes in Aqueous Suspensions"
        )));
    }

    #[test]
    fn rejects_a_different_record_about_the_same_paper() {
        assert!(!publication(
            "Author response for \"Development of Hybrid Nanoparticles Based on Zr(IV)\""
        )
        .is_its_metadata(&metadata(
            "Development of hybrid nanoparticles based on Zr(IV)"
        )));
    }

    #[test]
    fn rejects_a_short_incidental_overlap() {
        assert!(
            !publication("Silicon nanoparticles").is_its_metadata(&metadata(
                "Silicon nanoparticles for biological imaging applications"
            ))
        );
    }
}
