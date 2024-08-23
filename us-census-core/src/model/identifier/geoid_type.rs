use super::{fips, geoid::Geoid};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GeoidType {
    State,
    County,
    CountySubdivision,
    Place,
    CensusTract,
    BlockGroup,
    Block,
}

impl ToString for GeoidType {
    fn to_string(&self) -> String {
        match self {
            GeoidType::State => String::from("state"),
            GeoidType::County => String::from("county"),
            GeoidType::CountySubdivision => String::from("county subdivision"),
            GeoidType::Place => String::from("place"),
            GeoidType::CensusTract => String::from("census tract"),
            GeoidType::BlockGroup => String::from("block group"),
            GeoidType::Block => String::from("block"),
        }
    }
}

impl GeoidType {
    pub fn geoid_from_string(&self, value: &String) -> Result<Geoid, String> {
        match self {
            GeoidType::State => {
                if value.len() != 2 {
                    Err(format!(
                        "for state geoid, expected 2-digit value, found: {}",
                        value
                    ))
                } else {
                    self.geoid_from_slice_of_strings(&vec![value.to_string()])
                }
            }
            GeoidType::County => {
                if value.len() != 5 {
                    Err(format!(
                        "for county geoid, expected 5-digit value, found: {}",
                        value
                    ))
                } else {
                    self.geoid_from_slice_of_strings(&[
                        value[0..2].to_string(),
                        value[2..5].to_string(),
                    ])
                }
            }
            GeoidType::CountySubdivision => {
                if value.len() != 10 {
                    Err(format!(
                        "for county subdivision geoid, expected 10-digit value, found: {}",
                        value
                    ))
                } else {
                    self.geoid_from_slice_of_strings(&[
                        value[0..2].to_string(),
                        value[2..5].to_string(),
                        value[5..10].to_string(),
                    ])
                }
            }
            GeoidType::Place => {
                if value.len() != 7 {
                    Err(format!(
                        "for place geoid, expected 7-digit value, found: {}",
                        value
                    ))
                } else {
                    self.geoid_from_slice_of_strings(&[
                        value[0..2].to_string(),
                        value[2..7].to_string(),
                    ])
                }
            }
            GeoidType::CensusTract => {
                if value.len() != 11 {
                    Err(format!(
                        "for census tract geoid, expected 11-digit value, found: {}",
                        value
                    ))
                } else {
                    self.geoid_from_slice_of_strings(&[
                        value[0..2].to_string(),
                        value[2..5].to_string(),
                        value[5..11].to_string(),
                    ])
                }
            }
            GeoidType::BlockGroup => {
                if value.len() != 12 {
                    Err(format!(
                        "for block group geoid, expected 12-digit value, found: {}",
                        value
                    ))
                } else {
                    self.geoid_from_slice_of_strings(&[
                        value[0..2].to_string(),
                        value[2..5].to_string(),
                        value[5..11].to_string(),
                        value[11..12].to_string(),
                    ])
                }
            }
            GeoidType::Block => {
                if value.len() == 15 || value.len() != 16 {
                    Err(format!(
                        "for block geoid, expected 15 or 16-digit value, found: {}",
                        value
                    ))
                } else {
                    self.geoid_from_slice_of_strings(&[
                        value[0..2].to_string(),
                        value[2..5].to_string(),
                        value[5..11].to_string(),
                        value[11..].to_string(),
                    ])
                }
            }
        }
    }
    pub fn geoid_from_slice_of_strings(&self, vals: &[String]) -> Result<Geoid, String> {
        match self {
            GeoidType::State => {
                let arr = as_usizes(vals)?;
                if arr.len() != 1 {
                    Err(format!(
                        "for state-level query, expected 1 geoid column, found: {}",
                        arr.into_iter().join(",")
                    ))
                } else {
                    Ok(Geoid::State(fips::State(arr[0])))
                }
            }
            GeoidType::County => {
                let arr = as_usizes(vals)?;
                if arr.len() != 2 {
                    Err(format!(
                        "for county-level query, expected 2 geoid columns, found: {}",
                        arr.into_iter().join(",")
                    ))
                } else {
                    Ok(Geoid::County(fips::State(arr[0]), fips::County(arr[1])))
                }
            }
            GeoidType::CountySubdivision => {
                let arr = as_usizes(vals)?;
                if arr.len() != 3 {
                    Err(format!(
                        "for county subdivision-level query, expected 3 geoid columns, found: {}",
                        arr.into_iter().join(",")
                    ))
                } else {
                    Ok(Geoid::CountySubdivision(
                        fips::State(arr[0]),
                        fips::County(arr[1]),
                        fips::CountySubdivision(arr[2]),
                    ))
                }
            }
            GeoidType::Place => {
                let arr = as_usizes(vals)?;
                if arr.len() != 2 {
                    Err(format!(
                        "for place-level query, expected 2 geoid columns, found: {}",
                        arr.into_iter().join(",")
                    ))
                } else {
                    Ok(Geoid::Place(fips::State(arr[0]), fips::Place(arr[1])))
                }
            }
            GeoidType::CensusTract => {
                let arr = as_usizes(vals)?;
                if arr.len() != 3 {
                    Err(format!(
                        "for census tract-level query, expected 3 geoid column, found: {}",
                        arr.into_iter().join(",")
                    ))
                } else {
                    Ok(Geoid::CensusTract(
                        fips::State(arr[0]),
                        fips::County(arr[1]),
                        fips::CensusTract(arr[2]),
                    ))
                }
            }
            GeoidType::BlockGroup => {
                let arr = as_usizes(vals)?;
                if arr.len() != 4 {
                    Err(format!(
                        "for block group-level query, expected 4 geoid columns, found: {}",
                        arr.into_iter().join(",")
                    ))
                } else {
                    Ok(Geoid::BlockGroup(
                        fips::State(arr[0]),
                        fips::County(arr[1]),
                        fips::CensusTract(arr[2]),
                        fips::BlockGroup(arr[3]),
                    ))
                }
            }
            GeoidType::Block => {
                let arr = as_usizes(vals)?;
                if arr.len() != 5 {
                    Err(format!(
                        "for block group-level query, expected 4 geoid columns, found: {}",
                        arr.into_iter().join(",")
                    ))
                } else {
                    Ok(Geoid::Block(
                        fips::State(arr[0]),
                        fips::County(arr[1]),
                        fips::CensusTract(arr[2]),
                        fips::Block(format!("{}", arr[3])),
                    ))
                }
            }
        }
    }
}

/// helper function to convert a slice of strings into u64s used to build fips::* values.
fn as_usizes(arr: &[String]) -> Result<Vec<u64>, String> {
    arr.iter()
        .map(|v| {
            let v_u64 = v.parse::<u64>().map_err(|e| {
                format!(
                    "raw geoid value should be a string wrapping an integer, found '{}'. error: {}",
                    v, e
                )
            })?;
            Ok(v_u64)
        })
        .collect::<Result<Vec<u64>, String>>()
}
