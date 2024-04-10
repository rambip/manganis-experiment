# Description

This is an experimental project showing how you can use the `#[link_section]` attribute to pass data from a rust code downward the toolchain.

It could be used for [manganis](https://github.com/DioxusLabs/collect-assets), to pass data from the rust file to the dioxus cli.

# How to test

1) choose a target

```
export TARGET=WASM32-unknown-unknwown
```

2) build the "slave" code

```
cargo build --package slave --release --target $TARGET
```

3) find where it was generated:

```
find target slave -maxdepth 3 -type f -executable -name "slave*"
```


4) launch the master code with this file as argument:

```
 cargo run --package manganis-linkme -- <path of slave executable>
```

You should something like:
```
I have found this data for manganis:

    {
            "path": "/home/user/test/asset.txt",
            "options": "foo",
    }

    {
            "path": "/home/user/test/style.css",
            "options": "bar",
    }

```
