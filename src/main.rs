mod models;

mod config;
mod config_loader;
mod epub_packager;
mod html_compile;
mod meta_manage;
mod resource_mapping;

fn main() {
    println!("Hello, world!");
    let connections = config_loader::load_config();
}
