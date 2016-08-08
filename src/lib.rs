extern crate hyper;
extern crate xml;
pub mod abstractions;
pub mod entities;
pub mod services;
mod util;

use hyper::client::Client;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io::Read;
use std::str;
use xml::writer::{EmitterConfig};

use abstractions::traits::Transaction;
use entities::portico::{Header, PosRequest, Ver10};
use entities::soap::{SoapEnvelope, SoapBody};
use services::ServicesConfig;

pub fn connect<T: Transaction>(s: ServicesConfig, t: T) {
    let url = "https://cert.api2.heartlandportico.com\
               /Hps.Exchange.PosGateway\
               /PosGatewayService.asmx?wsdl";
    let body = build_xml(s, t);
    let body_str = str::from_utf8(&body).unwrap();
    println!("request: {}", body_str);

    let client = Client::new();
    let builder = client.post(url)
        .body(body_str)
        .header(ContentType(Mime(TopLevel::Text,
                                 SubLevel::Xml,
                                 vec![(Attr::Charset, Value::Utf8)])));

    match builder.send() {
        Ok(mut res) => {
            let mut buf = String::new();
            let _ = res.read_to_string(&mut buf)
                .map_err(|_| println!("Failed to read response body"))
                .map(|_| {
                    println!("code: {}\nbody: {:?}", res.status, buf);
                });
        }
        _ => {
            println!("Failed to get a response");
        }
    }
}

pub fn build_xml<T: Transaction>(s: ServicesConfig, t: T) -> Vec<u8> {
    let mut b = Vec::new();

    {
        let mut w = EmitterConfig::new()
            .write_document_declaration(true)
            .perform_indent(true)
            .create_writer(&mut b);

        SoapEnvelope {
            header: None,
            body: Some(SoapBody {
                contents: Some(PosRequest {
                    version: Some(Ver10 {
                        header: Some(Header {
                            device_id: s.device_id,
                            developer_id: s.developer_id,
                            username: s.username,
                            site_id: s.site_id,
                            license_id: s.license_id,
                            version_number: s.version_number,
                            secret_api_key: s.secret_api_key,
                            password: s.password,
                        }),
                        transaction: Some(t),
                    })
                })
            }),
        }.write_xml(&mut w);
    }

    b
}
