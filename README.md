<p align="center">
  <img width=100px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/f5cd30c4-d27b-4d38-b5f1-3bb4d47ed54a"
</p>

<hr>

# Rust's data ecosystem

- ndarray
  - https://github.com/rust-ndarray/ndarray
- tikv
  - https://github.com/tikv/tikv 
- Roapi
  - https://github.com/roapi/roapi
- Burn
  - https://github.com/tracel-ai/burn
- Databend
  - https://docs.databend.com/guides
- Materialize
  - https://materialize.com/
- Graphana SDK
  - https://github.com/grafana/grafana-plugin-sdk-rust
    - https://github.com/sd2k/grafana-sample-backend-plugin-rust/
- Polars
  - https://pola.rs/


<img src="https://camo.githubusercontent.com/46148d948a6b0364d9dcf48c64b9f21cbdd0a4e5550dfd8b24b317fef881eaf9/68747470733a2f2f726f6170692e6769746875622e696f2f646f63732f696d616765732f726f6170692e706e67" />

![Screenshot 2024-03-30 at 12 15 13 AM](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/25ad116b-51c5-42db-b9a9-46244324982c)

- Delta-rs, Apache Arrow, Polars, WASM: Is Rust the Future of Analytics? | Databricks
  - https://youtu.be/VedepVXiql4?si=nx5eVcgWy7m1_jvd

<hr>

# rust_finance_pola_rs


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

<hr>

# Rust Quant(Rust Finance)

- 여기에 정리중
  - https://github.com/YoungHaKim7/rust_finance_pola_rs

# Latex 문법

- https://junia3.github.io/blog/latex_symbols

<hr>

# 인터넷으로 파이썬 쥬피터노트북 해보기 (신기하네 ㅋ)
- https://jupyterlite.github.io/demo/lab/index.html

# Jupyter 노트북 러스트로 빠르게 돌리기

- https://racum.blog/articles/rust-jupyter/

- First, you need to download and build the kernel itself via cargo:

```bash
$ cargo install --locked evcxr_jupyter
```

- Then, use its binary to automatically install it inside Jupyter:

```bash
$ evcxr_jupyter --install
```

<hr>

# Next-gen Python tooling

- https://astral.sh/
- matplotlib
  - https://matplotlib.org/stable/tutorials/pyplot.html

# conda대체 - GN⁺: Uv - 러스트로 구현한 초고속 파이썬 패키징 도구 (astral.sh)
- https://news.hada.io/topic?id=13388
  - https://astral.sh/blog/uv

- uv(conda보다 가벼운 패키지)
  - https://pypi.org/project/uv/

<hr>
