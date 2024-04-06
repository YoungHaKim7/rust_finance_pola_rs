<p align="center">
  <img width=100px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/f5cd30c4-d27b-4d38-b5f1-3bb4d47ed54a"
</p>

# link

- [Rust러스트fishshell용-echo로-gitignore넣기](#rustfishshell용-echo로-gitignore넣기)
  - [Rust(.gitignore)복사해서 넣기](#rustgitignore그냥-넣기)
- [python파이썬.gitignore](#pythongitignore)
- [Mojo정리함(&MAX)](https://github.com/YoungHaKim7/Modular_Mojo_AI_Dev)

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

# python(.gitignore)[[🔝]](#link)

```gitignore
# A collection of useful .gitignore templates 
# https://github.com/github/gitignore\xa
# General
.DS_Store
dir/otherdir/.DS_Store
.ruff_cache/
.venv/


# Byte-compiled / optimized / DLL files
__pycache__/
*.py[cod]
*$py.class

# C extensions
*.so

# Distribution / packaging
.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
share/python-wheels/
*.egg-info/
.installed.cfg
*.egg
MANIFEST

# PyInstaller
#  Usually these files are written by a python script from a template
#  before PyInstaller builds the exe, so as to inject date/other infos into it.
*.manifest
*.spec

# Installer logs
pip-log.txt
pip-delete-this-directory.txt

# Unit test / coverage reports
htmlcov/
.tox/
.nox/
.coverage
.coverage.*
.cache
nosetests.xml
coverage.xml
*.cover
*.py,cover
.hypothesis/
.pytest_cache/
cover/

# Translations
*.mo
*.pot

# Django stuff:
*.log
local_settings.py
db.sqlite3
db.sqlite3-journal

# Flask stuff:
instance/
.webassets-cache

# Scrapy stuff:
.scrapy

# Sphinx documentation
docs/_build/

# PyBuilder
.pybuilder/
target/

# Jupyter Notebook
.ipynb_checkpoints

# IPython
profile_default/
ipython_config.py

# pyenv
#   For a library or package, you might want to ignore these files since the code is
#   intended to run in multiple environments; otherwise, check them in:
# .python-version

# pipenv
#   According to pypa/pipenv#598, it is recommended to include Pipfile.lock in version control.
#   However, in case of collaboration, if having platform-specific dependencies or dependencies
#   having no cross-platform support, pipenv may install dependencies that don't work, or not
#   install all needed dependencies.
#Pipfile.lock

# poetry
#   Similar to Pipfile.lock, it is generally recommended to include poetry.lock in version control.
#   This is especially recommended for binary packages to ensure reproducibility, and is more
#   commonly ignored for libraries.
#   https://python-poetry.org/docs/basic-usage/#commit-your-poetrylock-file-to-version-control
#poetry.lock

# pdm
#   Similar to Pipfile.lock, it is generally recommended to include pdm.lock in version control.
#pdm.lock
#   pdm stores project-wide configurations in .pdm.toml, but it is recommended to not include it
#   in version control.
#   https://pdm.fming.dev/#use-with-ide
.pdm.toml

# PEP 582; used by e.g. github.com/David-OConnor/pyflow and github.com/pdm-project/pdm
__pypackages__/

# Celery stuff
celerybeat-schedule
celerybeat.pid

# SageMath parsed files
*.sage.py

# Environments
.env
.venv
env/
venv/
ENV/
env.bak/
venv.bak/

# Spyder project settings
.spyderproject
.spyproject

# Rope project settings
.ropeproject

# mkdocs documentation
/site

# mypy
.mypy_cache/
.dmypy.json
dmypy.json

# Pyre type checker
.pyre/

# pytype static type analyzer
.pytype/

# Cython debug symbols
cython_debug/

# PyCharm
#  JetBrains specific template is maintained in a separate JetBrains.gitignore that can
#  be found at https://github.com/github/gitignore/blob/main/Global/JetBrains.gitignore
#  and can be added to the global gitignore or merged into this file.  For a more nuclear
#  option (not recommended) you can uncomment the following to ignore the entire idea folder.
#.idea/
```

# (Rust)fishshell용 echo로 gitignore넣기[[🔝]](#link)

```gitignore
echo "# Result" >> README.md &&
echo "" >> README.md &&
echo "```bash" >> README.md &&
echo "" >> README.md &&
echo "```" >> README.md &&
echo "" >> README.md &&

echo "# Visual Studio 2015/2017 cache/options directory" >> .gitignore &&
echo ".vs/" >> .gitignore &&
echo "" >> .gitignore &&
echo "# A collection of useful .gitignore templates " >> .gitignore &&
echo "# https://github.com/github/gitignore\xa" >> .gitignore &&
echo "# General" >> .gitignore &&
echo ".DS_Store" >> .gitignore &&
echo "dir/otherdir/.DS_Store" >> .gitignore &&
echo "" >> .gitignore &&

echo "# VS Code files for those working on multiple tools" >> .gitignore &&
echo ".vscode/" >> .gitignore &&
echo "# Generated by Cargo" >> .gitignore  &&
echo "# will have compiled files and executables" >> .gitignore &&
echo "debug/" >> .gitignore &&
echo "target/" >> .gitignore &&
echo "" >> .gitignore &&

echo "# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries" >> .gitignore &&
echo "# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html" >> .gitignore &&
echo "Cargo.lock" >> .gitignore &&
echo "" >> .gitignore &&

echo "# These are backup files generated by rustfmt" >> .gitignore &&
echo "**/*.rs.bk" >> .gitignore &&
echo "" >> .gitignore &&

echo "# MSVC Windows builds of rustc generate these, which store debugging information" >> .gitignore &&
echo "*.pdb" >> .gitignore &&
echo "" >> .gitignore &&

echo "# WASM" >> .gitignore &&
echo "pkg/" >> .gitignore &&
echo "/wasm-pack.log" >> .gitignore &&
echo "dist/" >> .gitignore
```

# Rust(.gitignore)그냥 넣기[[🔝]](#link)

- [Rust러스트fishshell용-echo로-gitignore넣기](#rustfishshell용-echo로-gitignore넣기)

```gitignore
# A collection of useful .gitignore templates 
# https://github.com/github/gitignore

# General
.DS_Store

# VS Code files for those working on multiple tools
.vscode/

# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb

# WASM
pkg/
/wasm-pack.log
dist/
```
