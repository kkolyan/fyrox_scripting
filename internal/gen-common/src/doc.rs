use itertools::Itertools;
use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};
use std::borrow::Cow;
use std::collections::HashMap;
use to_vec::ToVec;
use xml::name::{Name, OwnedName};
use xml::reader::XmlEvent;
use xml::writer::XmlEvent as WriterEvent;
use xml::{EmitterConfig, EventReader, EventWriter};

#[extend::ext]
pub impl str {
    fn to_doc(&self, indent: &str) -> String {
        self.md2html()
            .html2xmldoc()
            .lines()
            .map(|it| format!("\n{}/// {}", indent, it))
            .join("")
    }

    fn to_doc_commented(&self, indent: &str) -> String {
        self.md2html()
            .html2xmldoc()
            .lines()
            .map(|it| format!("\n{}// {}", indent, it))
            .join("")
    }

    fn md2html(&self) -> String {
        let parser = Parser::new_ext(self, Options::all());
        let mut html_output = String::new();
        push_html(&mut html_output, parser);
        html_output.trim().replace("<kbd>\\</kbd>", "<kbd>\\\\</kbd>").to_string()
    }

    fn html2xmldoc(&self) -> String {
        if self.is_empty() {
            return self.to_string();
        }

        let mapping: HashMap<&str, Vec<&str>> = HashMap::from_iter([
            ("p", vec!["para"]),
            ("code", vec!["c"]),
            ("kbd", vec!["c"]),
            ("pre", vec!["code"]),
            ("a", vec!["see"]),
            ("h1", vec!["para", "b"]),
            ("h2", vec!["para", "b"]),
            ("h3", vec!["para", "b"]),
            ("strong", vec!["b"]),
        ]);

        let mut out = vec![];
        let reader = EventReader::new(self.as_bytes());
        let mut w = EventWriter::new_with_config(
            &mut out,
            EmitterConfig::new().write_document_declaration(false),
        );
        for event in reader {
            match event {
                Ok(event) => match event {
                    XmlEvent::StartDocument {.. } => {}
                    XmlEvent::EndDocument => break,
                    XmlEvent::ProcessingInstruction { name, data } => {
                        w.write(WriterEvent::ProcessingInstruction {
                            name: name.as_str(),
                            data: data.as_deref(),
                        })
                        .unwrap();
                    }
                    XmlEvent::StartElement {
                        name,
                        attributes,
                        namespace,
                    } =>{
                        let x = &vec![name.local_name.as_str()];
                        let elements = mapping.get(name.local_name.as_str())
                            .unwrap_or(x);
                        for element in elements {
                            w
                                .write(WriterEvent::StartElement {
                                    name: Name::local(element),
                                    attributes: Cow::Owned(
                                        attributes.iter().map(|it| it.borrow()).to_vec(),
                                    ),
                                    namespace: Cow::Borrowed(&namespace),
                                })
                                .unwrap()
                        }
                    },
                    XmlEvent::EndElement { name } => {
                        let vec1 = vec![name.local_name.as_str()];
                        let elements = mapping.get(name.local_name.as_str())
                            .unwrap_or(&vec1);
                        for element in elements.iter().rev() {
                            w
                                .write(WriterEvent::EndElement {
                                    name: Some(Name::local(element)),
                                })
                                .unwrap()
                        }
                    },
                    XmlEvent::CData(it) => w.write(WriterEvent::CData(it.as_str())).unwrap(),
                    XmlEvent::Comment(it) => w.write(it.as_str()).unwrap(),
                    XmlEvent::Characters(it) => w.write(it.as_str()).unwrap(),
                    XmlEvent::Whitespace(it) => w.write(it.as_str()).unwrap(),
                },
                Err(err) => panic!("{:?}. source:\n{}", err, self),
            }
        }
        String::from_utf8(out).unwrap()
    }
}
