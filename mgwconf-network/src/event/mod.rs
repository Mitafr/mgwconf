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
    DeleteBusinessApplication,
    DeleteCertificate,
    DeleteSag,
}
