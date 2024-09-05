use clap::Parser;
use itertools::Itertools;
use us_census_acs::model::acs_type::AcsType;
use us_census_app::acs_tiger;
use us_census_app::model::acs_tiger_output_row::AcsTigerOutputRow;
use us_census_core::model::identifier::geoid::Geoid;
use us_census_core::model::identifier::geoid_type::GeoidType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AcsTigerAppCli {
    #[arg(short, long)]
    pub geoid: String,
    #[arg(short, long)]
    pub wildcard: Option<GeoidType>,
    #[arg(long)]
    pub year: u64,
    #[arg(long)]
    pub acs_query: String,
    #[arg(short, long)]
    pub acs_type: AcsType,
    #[arg(short, long)]
    pub acs_token: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = AcsTigerAppCli::parse();
    let acs_get_query = args.acs_query.split(',').map(String::from).collect_vec();
    let geoid = Geoid::try_from(args.geoid.as_str()).unwrap();

    let res = acs_tiger::run(
        args.year,
        args.acs_type,
        acs_get_query,
        Some(geoid),
        args.wildcard,
        args.acs_token,
    )
    .await
    .unwrap();
    println!(
        "found {} responses, {}/{}/{} errors",
        res.join_dataset.len(),
        res.acs_errors.len(),
        res.tiger_errors.len(),
        res.join_errors.len(),
    );
    // println!("RESULTS");
    // for row in res.join_dataset.into_iter() {
    //     println!("{}", row)
    // }
    println!("ACS ERRORS");
    for row in res.acs_errors.into_iter() {
        println!("{}", row)
    }
    println!("TIGER ERRORS");
    for row in res.tiger_errors.into_iter() {
        println!("{}", row)
    }
    println!("JOIN ERRORS");
    for row in res.join_errors.into_iter() {
        println!("{}", row)
    }
    let mut writer = csv::WriterBuilder::new().from_path("output.csv").unwrap();
    for row in res.join_dataset {
        let out_row = AcsTigerOutputRow::from(row);
        writer.serialize(out_row).unwrap();
    }
}
