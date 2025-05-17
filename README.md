# sqlite-shortid

## Installation

- Use [sqlpkg](https://sqlpkg.org/) to manage extensions.
- Install it with `sqlpkg fnando/sqlite-shortid`.

To test if everything worked, run:

```console
$ sqlite3 --cmd '.load .sqlpkg/fnando/sqlite-shortid/shortid' <<< 'select shortid();'
┌────────────┐
│ shortid()  │
├────────────┤
│ 9765TXUKG6 │
└────────────┘
Run Time: real 0.000 user 0.000020 sys 0.000049
```

## Usage

```sql
.load .sqlpkg/fnando/sqlite-shortid/shortid
.mode line
.header on

select shortid_version(),
       shortid(),
       shortid("S"),
       shortid("W"),
       shortid("", 20),
       shortid("U", 20);
```
