fn main() {
    embed_resource::compile("assets/DreadHunger.rc", embed_resource::NONE);
    built::write_built_file().expect("Failed to acquire build-time information");
}
