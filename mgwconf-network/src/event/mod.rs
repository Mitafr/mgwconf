use crate::models::configuration::{ApplicationProfileEntity, BusinessApplicationEntity, CertificateEntity, ForwardProxyEntity, SagEntity};

#[derive(Debug)]
pub enum IoEvent {
    Ping,
    GetAllForwardProxyEntity,
    GetAllBusinessApplications,
    GetAllApplicationProfileEntity,
    GetAllCertificates,
    GetAllSags,
    GetAllProfiles,
    PostBusinessApplication,
    PostForwardProxyEntity,
    PostProfile,
    PostCertificate,
    PostSag,
    DeleteForwardProxyEntity(ForwardProxyEntity),
    DeleteApplicationProfileEntity(ApplicationProfileEntity),
    DeleteBusinessApplication(BusinessApplicationEntity),
    DeleteCertificate(CertificateEntity),
    DeleteSag(SagEntity),
}
