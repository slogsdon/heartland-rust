extern crate xml;

use std::fmt;
use std::io::{Write};
use xml::writer::EventWriter;

use super::super::abstractions::traits::Transaction;
use super::super::util::xml::{start_element, end_element, maybe_write_value, write_value};

pub struct PosRequest<T: Transaction> {
    pub version: Option<Ver10<T>>,
}

impl<T: Transaction> fmt::Debug for PosRequest<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PosRequest {{ version: {:?} }}", self.version)
    }
}

impl<T: Transaction> Default for PosRequest<T> {
    fn default() -> Self {
        PosRequest {
            version: None,
        }
    }
}

impl<T: Transaction> Transaction for PosRequest<T> {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
        start_element("PosRequest", w);

        if let Some(ref version) = self.version {
            version.write_xml(w);
        }

        end_element(w);
    }
}

pub struct Ver10<T: Transaction> {
    pub header: Option<Header>,
    pub transaction: Option<T>,
}

impl<T: Transaction> fmt::Debug for Ver10<T>
where T: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ver10 {{ header: {:?}, transaction: {:?} }}", self.header, self.transaction)
    }
}

impl<T: Transaction> Default for Ver10<T> {
    fn default() -> Self {
        Ver10 {
            header: None,
            transaction: None,
        }
    }
}

impl<T: Transaction> Transaction for Ver10<T> {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
        start_element("Ver1.0", w);

        if let Some(ref header) = self.header {
            header.write_xml(w);
        }

        if let Some(ref transaction) = self.transaction {
            start_element("Transaction", w);
            transaction.write_xml(w);
            end_element(w);
        }

        end_element(w);
    }
}

#[derive(Default, Debug)]
pub struct Header {
    pub secret_api_key: Option<&'static str>,
    pub username: Option<&'static str>,
    pub password: Option<&'static str>,
    pub site_id: Option<&'static str>,
    pub device_id: Option<&'static str>,
    pub license_id: Option<&'static str>,
    pub developer_id: Option<&'static str>,
    pub version_number: Option<&'static str>,
}

impl Transaction for Header {
    fn write_xml<W: Write>(&self, w: &mut EventWriter<W>) {
        start_element("Header", w);

        if let Some(ref secret_api_key) = self.secret_api_key {
            write_value("SecretAPIKey", secret_api_key, w);
        } else {
            maybe_write_value("UserName", &self.username, w);
            maybe_write_value("Password", &self.password, w);
            maybe_write_value("SiteId", &self.site_id, w);
            maybe_write_value("DeviceId", &self.device_id, w);
            maybe_write_value("LicenseId", &self.license_id, w);
        }
        maybe_write_value("DeveloperID", &self.developer_id, w);
        maybe_write_value("VersionNbr", &self.version_number, w);

        end_element(w);
    }
}
