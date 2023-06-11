import polars as pl

import polars_geocode

df = pl.DataFrame(
    {
        "name": ["Tbilisi home ISP", "Google DNS", "Yandex", "Alibaba", "Unknown"],
        "ip": ["31.146.205.253", "8.8.8.8", "77.88.55.242", "140.205.174.2", None],
    }
).lazy()

p = polars_geocode.MaxmindParser("./tests/GeoLite2-City.mmdb")

print(
    df.with_columns(
        pl.col("ip")
        .map(
            p.ip_to_continent_country_city,
            return_dtype=polars_geocode.continent_city_country_dtype,
        )
        .alias("geo"),
    )
    .with_columns(
        pl.col("geo").struct.field("city").alias("city"),
    )
    .collect()
)

print(
    df.with_columns(
        pl.col("ip")
        .map(
            lambda x: polars_geocode.ip_to_continent_country_city(
                "./tests/GeoLite2-City.mmdb", x
            ),
            return_dtype=polars_geocode.continent_city_country_dtype,
        )
        .alias("geo"),
    )
    .with_columns(
        pl.col("geo").struct.field("city").alias("city"),
    )
    .collect()
)
