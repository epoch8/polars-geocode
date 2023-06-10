import polars as pl
import polars_geocode

df = pl.DataFrame(
    {
        "name": ["Tbilisi home ISP", "Google DNS", "Yandex", "Alibaba"],
        "ip": ["31.146.205.253", "8.8.8.8", "77.88.55.242", "140.205.174.2"],
    }
)

p = polars_geocode.MaxmindParser("./tests/GeoLite2-City.mmdb")

print(
    df.with_columns(
        pl.col("ip")
        .map(
            p.ip_to_continent_country_city
            # lambda x: polars_geocode.ip_to_continent_country_city(
            #     "./tests/GeoLite2-City.mmdb", x
            # )
        )
        .alias("geo"),
    )
)

print(
    df.with_columns(
        pl.col("ip")
        .map(
            lambda x: polars_geocode.ip_to_continent_country_city(
                "./tests/GeoLite2-City.mmdb", x
            )
        )
        .alias("geo"),
    )
)