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
    DeleteBusinessApplication(crate::model::business_application::Entity),
    DeleteCertificate(crate::model::certificate::Entity),
    DeleteSag(crate::model::sag::Entity),
}
