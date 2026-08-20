fn main() {
    embed_resource::compile("app.rc", embed_resource::NONE)
        .manifest_optional()
        .expect("Failed to embed resources");
}
