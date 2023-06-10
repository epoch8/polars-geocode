use polars::prelude::*;
use rayon::prelude::*;
use std::{net::IpAddr, str::FromStr};

fn _ip_to_ccc(
    reader: &maxminddb::Reader<Vec<u8>>,
    x: Option<&str>,
) -> Option<(Option<String>, Option<String>, Option<String>)> {
    match x {
        Some(x) => {
            let x = IpAddr::from_str(x).ok()?;
            let x = reader.lookup::<maxminddb::geoip2::City>(x).ok()?;

            Some((
                x.continent
                    .as_ref()
                    .and_then(|x| x.names.as_ref())
                    .and_then(|x| x.get("en").and_then(|x| Some(x.to_string()))),
                x.country
                    .as_ref()
                    .and_then(|x| x.names.as_ref())
                    .and_then(|x| x.get("en").and_then(|x| Some(x.to_string()))),
                x.city
                    .as_ref()
                    .and_then(|x| x.names.as_ref())
                    .and_then(|x| x.get("en").and_then(|x| Some(x.to_string()))),
            ))
        }
        _ => None,
    }
}

pub(super) fn ip_to_continent_country_city(
    database_filename: &str,
    ips: Series,
) -> PolarsResult<Series> {
    let reader = maxminddb::Reader::open_readfile(database_filename).unwrap();

    let ips_chunked = ips.utf8()?;

    let geos = ips_chunked
        .par_iter()
        .map(|x| _ip_to_ccc(&reader, x).unwrap_or((None, None, None)))
        .collect::<Vec<_>>();

    let (continent, country, city): (Vec<_>, Vec<_>, Vec<_>) = itertools::multiunzip(geos);

    StructChunked::new(
        "geo",
        &vec![
            Series::new("continent", continent),
            Series::new("country", country),
            Series::new("city", city),
        ],
    )
    .map(|ca| ca.into_series())
}
