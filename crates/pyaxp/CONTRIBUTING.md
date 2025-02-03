# Contributing

Same old, same old. Fork, clone, branch, commit, push, pull request. You know the drill.

Except for the python project, which uses [uv](https://docs.astral.sh/uv/) and it changes a few things.


# setup env
```bash
$ uv sync
Using CPython 3.13.1 interpreter at: /opt/homebrew/opt/python@3.13/bin/python3.13
Creating virtual environment at: .venv
Resolved 14 packages in 13ms
Installed 11 packages in 38ms
 + duckdb==1.1.3
 + iniconfig==2.0.0
 + numpy==2.2.2
 + packaging==24.2
 + pluggy==1.5.0
 + polars==1.21.0
 + py4j==0.10.9.7
 + pyarrow==19.0.0
 + pyaxp==0.1.9 (from file:///Users/jeroen/projects/yaxp/crates/pyaxp)
 + pyspark==3.5.4
 + pytest==8.3.4
$ 
```

# run tests
```bash
$ uv run pytest tests
============================================================================================== test session starts ==============================================================================================
platform darwin -- Python 3.13.1, pytest-8.3.4, pluggy-1.5.0
rootdir: /Users/moi/projects/yaxp/crates/pyaxp
configfile: pyproject.toml
collected 8 items

tests/test_arrow.py ..                                                                                                                                                                                    [ 25%]
tests/test_duckdb.py ..                                                                                                                                                                                   [ 50%]
tests/test_polars.py ..                                                                                                                                                                                   [ 75%]
tests/test_spark.py ..                                                                                                                                                                                    [100%]

=============================================================================================== 8 passed in 5.84s ===============================================================================================
$
```

# develop and test in python
```bash
$ maturin develop --uv
🔗 Found pyo3 bindings
🐍 Found CPython 3.13 at /Users/moi/projects/yaxp/crates/pyaxp/.venv/bin/python
📡 Using build options features from pyproject.toml
Audited 6 packages in 11ms
warning: /Users/moi/projects/yaxp/Cargo.toml: unused manifest key: workspace.package.name
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
📦 Built wheel for CPython 3.13 to /var/folders/gr/gl3fzn_n0_g4fzpcfv2g40gh0000gn/T/.tmpAsAqNn/pyaxp-0.1.9-cp313-cp313-macosx_11_0_arm64.whl
✏️  Setting installed package as editable
🛠 Installed pyaxp-0.1.9
$
```
```python
$ python
Python 3.13.1 (main, Dec  3 2024, 17:59:52) [Clang 16.0.0 (clang-1600.0.26.4)] on darwin
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyaxp import parse_xsd
>>> schema = parse_xsd("example.xsd", "polars")
>>> schema
{'Field1': String, 'Field2': String, 'Field3': String, 'Field4': String, 'Field5': Datetime(time_unit='ms', time_zone=None), 'Field6': Date, 'Field7': Date, 'Field8': String, 'Field9': String, 'Field10': String, 'Field11': String, 'Field12': Decimal(precision=25, scale=7), 'Field13': String, 'Field14': String, 'Field15': String, 'Field16': String, 'Field17': Date, 'Field18': String, 'Field19': String, 'Field20': Decimal(precision=38, scale=10), 'Field21': Int64}
>>>
$
```