use clap::{App, Arg};
use std::path::PathBuf;
use svg_tessellation_renderer::TriangleRenderer;

fn main() {
    let app = App::new("SVG Tessellation Renderer")
        .version("1.0")
        .author("Spencer C. Imbleau <spencer@imbleau.com>")
        .about("A basic renderer for SVGs using tessellation and little GPU features.")
        .arg(
            Arg::with_name("input")
                .help("An SVG file to render")
                .takes_value(true)
                .required(true)
                .index(1), // Args start at 1
        )
        .get_matches();

    // Get file
    let file_path: &PathBuf = &app.value_of("input").unwrap().into();
    let source = std::fs::read(file_path).expect("Input file cannot be read");
    let svg_contents = String::from_utf8(source).unwrap();
    // Run demo
    let mut renderer = TriangleRenderer::new();
    renderer.init_with_svg(&svg_contents).unwrap();
    renderer.run().unwrap();
}
