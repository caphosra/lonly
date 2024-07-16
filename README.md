# lonly

*A yet another logic programming language.*

```
num(z)
num(s($n)) <- num($n)
?num(s(s(s(z))))
```

```
add(z, $x, $x)
add(s($x), $y, s($z)) <- add($x, $y, $z)
?add($x, $y, s(s(s(z))))
```
