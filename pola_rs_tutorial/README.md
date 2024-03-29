# Who uses arrow2

- https://docs.databend.com/guides
- Materialize https://materialize.com/
- Graphana SDK https://github.com/grafana/grafana-plugin-sdk-rust
  - https://github.com/sd2k/grafana-sample-backend-plugin-rust/
- Polars https://pola.rs/



<hr>


# https://pola.rs/

- https://docs.pola.rs/user-guide/getting-started/#installing-polars

- Cargo.toml(`cargo add polars -F lazy`)

```toml
# Or Cargo.toml
[dependencies]
polars = { version = "x", features = ["lazy", ...]}

```

```rs
use std::fs::File;

use chrono::prelude::*;

let mut df: DataFrame = df!(
    "integer" => &[1, 2, 3],
    "date" => &[
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
    ],
    "float" => &[4.0, 5.0, 6.0],
    "string" => &["a", "b", "c"],
)
.unwrap();
println!("{}", df);
```


<hr>

