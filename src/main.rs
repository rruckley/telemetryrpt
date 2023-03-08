use std::io::BufWriter;

use pdf::error::PdfError;



fn main() -> Result<(),PdfError> {

    let font_family = genpdf::fonts::from_files("./fonts","ShantellSans-VariableFont_BNCE,INFM,SPAC,wght",None)
        .expect("Could not load font family");

    let mut doc = genpdf::Document::new(font_family);

    doc.set_title("Telemetry Report");
    let mut decorator = genpdf::SimplePageDecorator::new();

    doc.set_page_decorator(decorator);

    doc.push(genpdf::elements::Paragraph::new("Telemetry reporting from Grafana."));

    //let mut writer = BufWriter::new(inner)
    let mut buff : Vec<u8> = vec![];

    let w = BufWriter::new(buff);

    //let pdf = doc.render(w);
    doc.render_to_file("output.pdf").expect("Failed to write PDF file");

    Ok(())
}
