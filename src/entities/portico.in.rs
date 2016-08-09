pub mod portico {
    #![allow(unused_mut)]
    extern crate xml;

    use std::fmt;
    use std::io::{Write};
    use xml::writer::EventWriter;

    use super::super::abstractions::traits::Transaction;
    use super::super::services::ServicesConfig;
    use super::super::util::xml::{start_element, end_element, maybe_write_value, write_value};

    #[derive(Deserialize)]
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

    #[derive(Deserialize)]
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

    #[derive(Default, Debug, Deserialize)]
    pub struct Header {
        pub secret_api_key: Option<String>,
        pub username: Option<String>,
        pub password: Option<String>,
        pub site_id: Option<String>,
        pub device_id: Option<String>,
        pub license_id: Option<String>,
        pub developer_id: Option<String>,
        pub version_number: Option<String>,
    }

    impl Header {
        pub fn from_services_config(s: ServicesConfig) -> Self {
            let device_id = match s.device_id {
                Some(device_id) => Some(String::from(device_id)),
                None => None,
            };
            let developer_id = match s.developer_id {
                Some(developer_id) => Some(String::from(developer_id)),
                None => None,
            };
            let username = match s.username {
                Some(username) => Some(String::from(username)),
                None => None,
            };
            let site_id = match s.site_id {
                Some(site_id) => Some(String::from(site_id)),
                None => None,
            };
            let license_id = match s.license_id {
                Some(license_id) => Some(String::from(license_id)),
                None => None,
            };
            let version_number = match s.version_number {
                Some(version_number) => Some(String::from(version_number)),
                None => None,
            };
            let secret_api_key = match s.secret_api_key {
                Some(secret_api_key) => Some(String::from(secret_api_key)),
                None => None,
            };
            let password = match s.password {
                Some(password) => Some(String::from(password)),
                None => None,
            };
            Header {
                device_id: device_id,
                developer_id: developer_id,
                username: username,
                site_id: site_id,
                license_id: license_id,
                version_number: version_number,
                secret_api_key: secret_api_key,
                password: password,
            }
        }
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
}
