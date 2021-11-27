# python call modeule named query power by rust

## brefore to use
<details>
<summary>build python module</summary>

<summary>

```bash
python3 -m venv .env
source .env/bin/activate
pip install maturin ipython
maturin develop
```
</details>

## how to use

`$ ipython`
```python
In [1]: import queryer_py

In [2]: sql = queryer_py.example_sql()

In [3]: print(queryer_py.query(sql, 'csv'))
name,total_cases,new_cases,total_deaths,new_deathsIndia,32649947.0,46759.0,437370.0,509.0Iran,4869414.0,36279.0,105287.0,571.0Africa,7695475.0,33957.0,193394.0,764.0South America,36768062.0,33853.0,1126593.0,1019.0Brazil,20703906.0,27345.0,578326.0,761.0Mexico,3311317.0,19556.0,257150.0,863.0
```