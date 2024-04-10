macro_rules! bake_asset {
    ($name: ident, $value: expr) => {
        #[link_section = "manganis"]
        #[no_mangle]
        #[used]
        static $name: [u8; $value.len()] = *$value;
    };
}

bake_asset!(
    ASSET1,
    br#"
    {
        "path": "/home/user/test/asset.txt",
        "options": "foo",
    }
    "#
);

bake_asset!(
    ASSET2,
    br#"
    {
        "path": "/home/user/test/style.css",
        "options": "bar",
    }
    "#
);

pub fn show_paths() {
    println!("{:?}", std::str::from_utf8(&ASSET1));
    println!("{:?}", std::str::from_utf8(&ASSET2));
}

fn main() {
    show_paths()
}
