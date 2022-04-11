# rbasename

It was inspired in [GNU coreutils](https://github.com/coreutils/coreutils) `basename` program.

The purpose of this project is to learn about rust.

rbasename strip directories from filename and trailing suffix if specified.
 
### usage

```sh
rbasename NAME [SUFFIX]
```

```sh
rbasename /users/myusername/projects/rust/src/main.rs
```

output: `main.rs`

```sh
rbasename include/stdio.h .h
```

output: `stdio`

### test

```
make test
```

### build

```
make build
```

### install

You need to execute as root because it copies the binary to `/usr/local/bin`.

```
$ make install
```

### test, build and install

```
$ make all [prod=1]
```

If specified, `prod=1` argument will add `--release` flag in build stage and will copy the generated binary of ./target/release directory instead debug directory.

---
