.load target/debug/libsqlite_shortid
.mode line
.header on

select shortid_version(),
       shortid(),
       shortid("S"),
       shortid("W"),
       shortid("", 20),
       shortid("U", 20);
