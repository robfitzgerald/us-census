use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::{LodesEdition, LodesJobType, OdPart, WorkplaceSegment, BASE_URL};

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum LodesDataset {
    OD {
        edition: LodesEdition,
        job_type: LodesJobType,
        od_part: OdPart,
        year: u64,
    },
    RAC,
    WAC {
        edition: LodesEdition,
        job_type: LodesJobType,
        segment: WorkplaceSegment,
        year: u64,
    },
}

impl Display for LodesDataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl LodesDataset {
    pub fn description(&self) -> String {
        match self {
            LodesDataset::OD {
                edition,
                job_type,
                od_part,
                year,
            } => {
                format!("{} {} {} Origin-Destination data, {} job totals are associated with both a home Census Block and a work Census Block", year, edition, od_part, job_type)
            }
            LodesDataset::RAC => String::from(
                "Residence Area Characteristic data, jobs are totaled by home Census Block",
            ),
            LodesDataset::WAC {
                edition,
                job_type,
                segment,
                year,
            } => format!(
                "{} {} {} Workplace Area Characteristic data, {} jobs are totaled by work Census Block",
                year, edition, segment, job_type
            ),
        }
    }

    pub fn dataset_directory(&self) -> String {
        match self {
            LodesDataset::OD {
                edition: _,
                job_type: _,
                od_part: _,
                year: _,
            } => todo!(),
            LodesDataset::RAC => todo!(),
            LodesDataset::WAC {
                edition: _,
                job_type: _,
                segment: _,
                year: _,
            } => String::from("wac"),
        }
    }

    /// creates a URI to a LODES datasets based on the directory and file
    /// naming conventions described in the LODESTechDoc8.1.pdf file.
    /// see https://lehd.ces.census.gov/data/lodes/LODES8/LODESTechDoc8.1.pdf
    pub fn create_uri(&self, state_code: &str) -> String {
        match self {
            LodesDataset::OD {
                edition,
                job_type,
                od_part,
                year,
            } => {
                let filename =
                    format!("{}_od_{}_{}_{}.csv.gz", state_code, od_part, job_type, year);
                format!(
                    "{}/{}/{}/{}/{}",
                    BASE_URL,
                    edition,
                    state_code,
                    self.dataset_directory(),
                    filename
                )
            }
            LodesDataset::RAC => todo!(),
            LodesDataset::WAC {
                edition,
                job_type,
                segment,
                year,
            } => {
                let filename = format!(
                    "{}_wac_{}_{}_{}.csv.gz",
                    state_code, segment, job_type, year
                );
                format!(
                    "{}/{}/{}/{}/{}",
                    BASE_URL,
                    edition,
                    state_code,
                    self.dataset_directory(),
                    filename
                )
            }
        }
    }
}
