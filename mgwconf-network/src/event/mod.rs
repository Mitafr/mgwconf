use crate::models::configuration::{BusinessApplicationEntity, CertificateEntity, SagEntity};

#[derive(Debug)]
pub enum IoEvent {
    Ping,
    GetAllBusinessApplications,
    GetAllCertificates,
    GetAllSags,
    GetAllProfiles,
    PostBusinessApplication,
    PostProfile,
    PostCertificate,
    PostSag,
    DeleteBusinessApplication(BusinessApplicationEntity),
    DeleteCertificate(CertificateEntity),
    DeleteSag(SagEntity),
}
