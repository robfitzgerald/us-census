use crate::model::acs_tiger_row::AcsTigerRow;
use geo::Geometry;
use itertools::Itertools;
use reqwest::Client;
use us_census_acs::api::acs_api;
use us_census_acs::model::acs_api_query_params::AcsApiQueryParams;
use us_census_core::model::identifier::geoid::Geoid;
use us_census_tiger::model::tiger_uri_builder::TigerUriBuilder;
use us_census_tiger::ops::tiger_api;

pub struct AcsTigerResponse {
    pub join_dataset: Vec<AcsTigerRow>,
    pub tiger_errors: Vec<String>,
    pub join_errors: Vec<String>,
}

/// runs a query to ACS. the result will include a list of GEOIDs alongside
/// ACS data. all GEOIDs are used to run a set of downloads from the TIGER/Lines
/// datasets. the geometries from TIGER are combined with the ACS data producing
/// AcsTigerRows.
///
/// # Example
///
/// ```ignore
/// use us_census_app::acs_tiger;
/// use us_census_acs::model::acs_type::AcsType;
/// use us_census_core::model::identifier::geoid::Geoid;
/// use us_census_core::model::identifier::geoid_type::GeoidType;
/// use us_census_core::model::identifier::fips;
///
/// let year = 2020;
/// let acs_type = AcsType::FiveYear;
/// let acs_get_query = vec![String::from("NAME"), String::from("B01001_001E")];
/// let geoid = Geoid::State(fips::State(08));
/// let wildcard = GeoidType::County;
///
/// # tokio_test::block_on(async {
///     let res = acs_tiger::run(year, acs_type, acs_get_query, Some(geoid), Some(wildcard), None).await.unwrap();
///     println!(
///         "found {} responses, {}/{}/{} errors",
///         res.join_dataset.len(),
///         res.acs_errors.len(),
///         res.tiger_errors.len(),
///         res.join_errors.len(),
///     );
///     for row in res.join_dataset.into_iter() {
///         println!("{}", row)
///     }
/// # })
///
/// ```
pub async fn run(query: &AcsApiQueryParams) -> Result<AcsTigerResponse, String> {
    let client: Client = Client::new();

    let acs_rows = acs_api::batch_run(&client, vec![&query]).await?;

    // execute TIGER/Lines downloads
    let tiger_uri_builder = TigerUriBuilder::new(query.year)?;
    let geoids = &acs_rows.iter().map(|(geoid, _)| geoid).collect_vec();
    let tiger_response = tiger_api::run(&client, &tiger_uri_builder, geoids).await?;

    type NestedResult = (Vec<Vec<(Geoid, Geometry<f64>)>>, Vec<String>);
    let (tiger_rows_nested, tiger_errors): NestedResult =
        tiger_response.into_iter().partition_result();

    let (join_dataset, join_errors) =
        crate::ops::join::dataset_with_geometries(acs_rows, tiger_rows_nested)?;
    let output_dataset = join_dataset
        .into_iter()
        .flat_map(|(geoid, geometry, acs_values)| {
            acs_values
                .into_iter()
                .map(move |acs_value| AcsTigerRow::new(geoid.clone(), acs_value, geometry.clone()))
        })
        .collect_vec();

    let result = AcsTigerResponse {
        join_dataset: output_dataset,
        tiger_errors,
        join_errors,
    };
    Ok(result)
}