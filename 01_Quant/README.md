<p align="center">
  <img width=100px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/f5cd30c4-d27b-4d38-b5f1-3bb4d47ed54a"
</p>

<hr>

# Rust Quant(Rust Finance)

# Latex 문법

- https://junia3.github.io/blog/latex_symbols

# matplotlib tutorial

https://matplotlib.org/stable/tutorials/pyplot.html

- https://matplotlib.org/stable/users/explain/quick_start.html#

<hr>

# conda 활성화/비활성화

```
# 처음에 프로젝트 폴더에 활성화 하고 싶을때
conda activate

# 프로젝트 끝나고 폴더에 활성화 하고 싶을때
conda deactivate  
```

# Linux에 Anaconda 설치하고 fishshell에 적용하기

- `conda init fish`
```bash
conda init fish                                                                                             0 (0.004s)
no change     /home/gy/anaconda3/condabin/conda
no change     /home/gy/anaconda3/bin/conda
no change     /home/gy/anaconda3/bin/conda-env
no change     /home/gy/anaconda3/bin/activate
no change     /home/gy/anaconda3/bin/deactivate
no change     /home/gy/anaconda3/etc/profile.d/conda.sh
no change     /home/gy/anaconda3/etc/fish/conf.d/conda.fish
no change     /home/gy/anaconda3/shell/condabin/Conda.psm1
no change     /home/gy/anaconda3/shell/condabin/conda-hook.ps1
no change     /home/gy/anaconda3/lib/python3.11/site-packages/xontrib/conda.xsh
no change     /home/gy/anaconda3/etc/profile.d/conda.csh
modified      /home/gy/.config/fish/config.fish

==> For changes to take effect, close and re-open your current shell. <==
```

- https://stackoverflow.com/questions/34280113/add-conda-to-path-in-fish

- 자동으로 conda활성화 되는거 false로 만들기 (귀찮네)
```bash
conda config --set auto_activate_base false

```


- `anaconda/bin` PATH잡아주기
  - https://m31phy.tistory.com/16

# 파이썬 패키지 관리 프로그램 최신 업데이트

```bash
pip install --upgrade pip

# python3 업데이트
pip3 install --upgrade pip

python -m pip install --user --upgrade pip
```

- https://www.geeksforgeeks.org/python-pip/

- https://stackoverflow.com/questions/70622936/how-to-use-upgraded-pip-from-site-packages

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
