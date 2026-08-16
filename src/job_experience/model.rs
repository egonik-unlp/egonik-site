//! Model for the CV view exported by the `project-cv` pipeline.
//!
//! Mirrors `view.schema.json` (schemaVersion 1.0.0). Field optionality and the
//! enum variants come from the schema, not from any one `cv.json` sample, so the
//! collections that happen to be empty today are still typed.

use serde::{Deserialize, Serialize};

/// Year, year-month or full date: "2016", "2016-07", "2016-07-01".
pub type PartialDate = String;

/// Text in both site languages; both are always present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bilingual {
    pub en: String,
    pub es: String,
}

impl Bilingual {
    /// The text for `lang`, falling back to English for anything unrecognised.
    pub fn get(&self, lang: &str) -> &str {
        match lang {
            "es" => &self.es,
            _ => &self.en,
        }
    }
}

/// A span of time. `end: None` with `ongoing: true` means "present".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Period {
    pub start: PartialDate,
    pub end: Option<PartialDate>,
    pub ongoing: bool,
}

/// How far the source documents agree about a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Confidence {
    Confirmed,
    SingleSource,
    Disputed,
}

/// Whether something reaches the public export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ViewLanguage {
    En,
    Es,
    Both,
}

/// One arrangement of the databank.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct View {
    pub key: String,
    pub label: Bilingual,
    pub description: Option<Bilingual>,
    pub order: i64,
    pub language: ViewLanguage,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub city: String,
    pub province: Option<String>,
    pub country: String,
}

/// Who the CV is about.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Profile {
    pub given_name: String,
    pub family_name: String,
    pub full_name: String,
    pub headline: Bilingual,
    pub birth_date: Option<PartialDate>,
    pub birth_place: Option<String>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ContactKind {
    Email,
    Phone,
    Address,
    NationalId,
    Website,
}

/// One way of reaching the person.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContactPoint {
    pub kind: ContactKind,
    pub label: String,
    pub value: String,
    pub visibility: Visibility,
}

/// An external profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub platform: String,
    pub url: String,
}

/// A band of one view.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SectionGroup {
    pub key: String,
    pub view: String,
    pub label: Bilingual,
    pub order: i64,
}

/// A titled block of one view, and the records it draws.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    pub key: String,
    pub view: String,
    pub label: Bilingual,
    pub order: i64,
    pub group: Option<String>,
    pub contains: Vec<String>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub note: Option<Bilingual>,
    pub collapsed: bool,
    pub visibility: Visibility,
}

/// One paragraph of the personal statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummaryBlock {
    pub key: String,
    pub order: i64,
    pub text: Bilingual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PositionKind {
    Industry,
    Academic,
    Volunteer,
}

/// A job or engagement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub key: String,
    pub organization: String,
    pub role: Bilingual,
    pub kind: PositionKind,
    pub period: Period,
    pub description: Option<Bilingual>,
    pub technologies: Vec<String>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Dedication {
    Simple,
    SemiExclusive,
    Exclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppointmentCharacter {
    Interino,
    Ordinario,
    Transitorio,
    AdHonorem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Resolution {
    pub number: String,
    pub date: PartialDate,
}

/// A formal university appointment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcademicAppointment {
    pub key: String,
    pub title: Bilingual,
    pub dedication: Dedication,
    pub character: AppointmentCharacter,
    pub resolution: Option<Resolution>,
    pub chair: Option<String>,
    pub institution: String,
    pub period: Period,
    pub notes: Option<Bilingual>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EducationLevel {
    Technical,
    Undergraduate,
    Doctorate,
    PostgraduateCourse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompletionStatus {
    Completed,
    InProgress,
}

/// A degree or course of study.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Education {
    pub key: String,
    pub degree: Bilingual,
    pub level: EducationLevel,
    pub institution: String,
    pub faculty: Option<String>,
    pub period: Period,
    pub status: CompletionStatus,
    pub grade_average: Option<f64>,
    pub grade_average_basis: Option<String>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThesisLevel {
    Undergraduate,
    Doctorate,
}

/// An undergraduate or doctoral thesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Thesis {
    pub key: String,
    pub level: ThesisLevel,
    pub title: Bilingual,
    pub institution: String,
    pub advisors: Option<Vec<String>>,
    pub status: CompletionStatus,
    pub grade: Option<f64>,
    pub confidence: Confidence,
}

/// A funded research position.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Fellowship {
    pub key: String,
    pub name: Bilingual,
    pub grantor: String,
    pub host: String,
    pub by_competition: bool,
    pub period: Period,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CourseOutcome {
    Passed,
    Attended,
}

/// A course taken.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Course {
    pub key: String,
    pub title: Bilingual,
    pub institution: String,
    pub instructor: Option<String>,
    pub period: Period,
    pub hours: Option<i64>,
    pub grade: Option<f64>,
    pub outcome: CourseOutcome,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CefrLevel {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
}

/// An external certification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Certification {
    pub key: String,
    pub name: String,
    pub issuer: String,
    pub grade: Option<String>,
    pub cefr_level: Option<CefrLevel>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PublicationType {
    JournalArticle,
    Review,
    BookChapter,
    Proceedings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PublicationStatus {
    Published,
    Accepted,
    Submitted,
}

/// A published or submitted work.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Publication {
    pub key: String,
    #[serde(rename = "type")]
    pub kind: PublicationType,
    pub title: String,
    pub authors: Vec<String>,
    pub venue: String,
    pub year: Option<i64>,
    pub volume: Option<String>,
    pub pages: Option<String>,
    pub place: Option<String>,
    pub peer_reviewed: bool,
    pub status: PublicationStatus,
    pub doi: Option<String>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConferenceRole {
    PosterPresenter,
    CoAuthor,
    Speaker,
    Attendee,
}

/// A congress, workshop or symposium.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConferenceItem {
    pub key: String,
    pub event: String,
    pub role: ConferenceRole,
    pub contribution: Option<String>,
    pub co_authors: Vec<String>,
    pub place: String,
    pub period: Period,
    pub confidence: Confidence,
}

/// A distinction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Award {
    pub key: String,
    pub name: Bilingual,
    pub placement: Option<String>,
    pub organizer: String,
    pub date: PartialDate,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SupervisionKind {
    UndergraduateThesis,
    StudentFellowship,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SupervisionRole {
    Director,
    CoDirector,
}

/// A student directed or co-directed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Supervision {
    pub key: String,
    pub kind: SupervisionKind,
    pub role: SupervisionRole,
    pub student: String,
    pub topic: Option<Bilingual>,
    pub institution: String,
    pub year: i64,
    pub grade: Option<f64>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkillLevel {
    Primary,
    Working,
    Familiar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub name: String,
    pub level: SkillLevel,
}

/// A named group of technologies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SkillGroup {
    pub key: String,
    pub label: Bilingual,
    pub items: Vec<Skill>,
}

/// Aggregate counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Metrics {
    pub journal_publications: Option<i64>,
    pub congress_presentations: Option<i64>,
    pub grants_as_participant: Option<i64>,
    pub grants_as_lead: Option<i64>,
    pub teaching_years: Option<f64>,
    pub research_years: Option<f64>,
}

/// The root of `cv.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CvView {
    pub schema_version: String,
    pub generated_at: String,
    pub view: View,
    pub profile: Profile,
    pub contact: Vec<ContactPoint>,
    pub links: Vec<Link>,
    pub groups: Vec<SectionGroup>,
    pub sections: Vec<Section>,
    pub summary: Vec<SummaryBlock>,
    pub positions: Vec<Position>,
    pub appointments: Vec<AcademicAppointment>,
    pub education: Vec<Education>,
    pub theses: Vec<Thesis>,
    pub fellowships: Vec<Fellowship>,
    pub courses: Vec<Course>,
    pub certifications: Vec<Certification>,
    pub publications: Vec<Publication>,
    pub conferences: Vec<ConferenceItem>,
    pub awards: Vec<Award>,
    pub supervision: Vec<Supervision>,
    pub skills: Vec<SkillGroup>,
    pub metrics: Option<Metrics>,
}
