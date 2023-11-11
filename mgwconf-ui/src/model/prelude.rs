pub use log::debug;
use mgwconf_network::models::configuration::*;
pub use std::fmt::Write;

use crate::ui::fmt::FmtModel;

impl FmtModel for SagEntity {
    fn to_string(&self) -> String {
        let mut model = String::new();
        write!(
            model,
            r#"
Hostname : {}
Port : {}
LAU KEY : {}
SSL DN : {}
Message Partner : {}
User DNs : {}
            "#,
            self.hostname,
            self.port,
            self.lau_key.as_ref().unwrap_or(&"NULL".to_owned()),
            self.ssl_dn.as_ref().unwrap_or(&"NULL".to_owned()),
            self.message_partner_name.as_ref().unwrap_or(&"NULL".to_owned()),
            self.user_dns.join(" ")
        )
        .unwrap();
        model
    }
}

impl FmtModel for CertificateEntity {
    fn to_string(&self) -> String {
        let mut model = String::new();
        write!(
            model,
            r#"
Alias : {}
CertificateX509 : {}
        "#,
            self.alias, self.certificate_x509
        )
        .unwrap();
        model
    }
}

impl FmtModel for BusinessApplicationEntity {
    fn to_string(&self) -> String {
        let mut model = String::new();
        write!(model, "{}\n{}", self.application_name, self.shared_secret.as_ref().unwrap()).unwrap();
        model
    }
}
