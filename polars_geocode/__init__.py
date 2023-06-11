import polars as pl

from polars_geocode._polars_geocode import *

continent_city_country_dtype = pl.Struct(
    [
        pl.Field("continent", pl.Utf8),
        pl.Field("country", pl.Utf8),
        pl.Field("city", pl.Utf8),
    ]
)
