mod polars_geocode_mod;

use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::error::PyPolarsErr;
use pyo3_polars::PySeries;

#[pyfunction]
fn ip_to_continent_country_city(database_filename: &str, pyser: PySeries) -> PyResult<PySeries> {
    let ser: Series = pyser.into();

    let res = polars_geocode_mod::ip_to_continent_country_city(database_filename, ser)
        .map_err(PyPolarsErr::from)?;

    Ok(PySeries(res))
}

#[pymodule]
fn _polars_geocode(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ip_to_continent_country_city, m)?)?;
    Ok(())
}
