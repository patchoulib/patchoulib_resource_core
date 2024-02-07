mod models;

mod config;
mod html_compile;
mod epub_packager;
mod meta_manage;
mod resource_mapping;
mod config_loader;

fn main() {
    println!("Hello, world!");
    let connections = config_loader::load_config();
}
