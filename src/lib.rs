mod polars_geocode_mod;

use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::error::PyPolarsErr;
use pyo3_polars::PySeries;

#[pyclass(frozen)]
struct MaxmindParser {
    reader: maxminddb::Reader<Vec<u8>>,
}

#[pymethods]
impl MaxmindParser {
    #[new]
    fn new(database_filename: &str) -> Self {
        MaxmindParser {
            reader: maxminddb::Reader::open_readfile(database_filename).unwrap(),
        }
    }

    pub fn ip_to_continent_country_city(&self, pyser: PySeries) -> PyResult<PySeries> {
        let ser: Series = pyser.into();

        let res = polars_geocode_mod::ip_to_continent_country_city(&self.reader, ser)
            .map_err(PyPolarsErr::from)?;

        Ok(PySeries(res))
    }
}

#[pyfunction]
fn ip_to_continent_country_city(database_filename: &str, pyser: PySeries) -> PyResult<PySeries> {
    let reader = maxminddb::Reader::open_readfile(database_filename).unwrap();

    let ser: Series = pyser.into();

    let res = polars_geocode_mod::ip_to_continent_country_city(&reader, ser)
        .map_err(PyPolarsErr::from)?;

    Ok(PySeries(res))
}

#[pymodule]
fn _polars_geocode(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MaxmindParser>()?;
    m.add_function(wrap_pyfunction!(ip_to_continent_country_city, m)?)?;
    Ok(())
}
