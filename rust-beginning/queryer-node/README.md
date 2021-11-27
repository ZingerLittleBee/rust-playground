# node call modeule named query power by rust

## how to build
`yarn build` -> `index.node`

## how to use

```node
> const q = require('.')
undefined
> const sql = q.example_sql()
undefined
> sql
'SELECT location name, total_cases, new_cases, total_deaths, new_deaths FROM https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5'
> console.log(q.query(sql))
name,total_cases,new_cases,total_deaths,new_deaths
India,32649947.0,46759.0,437370.0,509.0
Iran,4869414.0,36279.0,105287.0,571.0
Africa,7695475.0,33957.0,193394.0,764.0
South America,36768062.0,33853.0,1126593.0,1019.0
Brazil,20703906.0,27345.0,578326.0,761.0
Mexico,3311317.0,19556.0,257150.0,863.0
```

## Learn More

To learn more about Neon, see the [Neon documentation](https://neon-bindings.com).

To learn more about Rust, see the [Rust documentation](https://www.rust-lang.org).

To learn more about Node, see the [Node documentation](https://nodejs.org).