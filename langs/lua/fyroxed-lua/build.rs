fn main() {
    to_ico::process_icon(
        "../../../internal/icons/fyrox_lua_003.png",
        "../../../target/fyrox_lua_003.ico",
    );
    embed_resource::compile("icon.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
