# Result

```bash

$ ruff format main.py

1 file reformatted

$ ruff check main.py

All checks passed!
```


```
$ uv venv

Using Python 3.12.2 interpreter at: /opt/homebrew/opt/python@3.12/bin/python3.12
Creating virtualenv at: .venv
Activate with: source .venv/bin/activate.fish


$ uv pip sync requirements.txt
Installed 11 packages in 36ms
 + contourpy==1.2.0
 + cycler==0.12.1
 + fonttools==4.50.0
 + kiwisolver==1.4.5
 + matplotlib==3.8.3
 + numpy==1.26.4
 + packaging==24.0
 + pillow==10.2.0
 + pyparsing==3.1.2
 + python-dateutil==2.9.0.post0
 + six==1.16.0



./.venv/bin/python3.12 main.py  
```
