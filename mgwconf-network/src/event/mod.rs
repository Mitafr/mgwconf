use crate::model::configuration::{ApiCredentialsEntity, ApiGatewayInfoEntity, ApplicationProfileEntity, BusinessApplicationEntity, CertificateEntity, ForwardProxyEntity, SagEntity};

#[derive(Debug, Clone)]
pub enum IoEvent {
    Ping,
    GetAllApiGatewayInfoEntity,
    GetAllForwardProxyEntity,
    GetAllApiClientCredentials,
    GetAllBusinessApplications,
    GetAllCertificates,
    GetAllSags,
    GetAllProfiles,
    PostBusinessApplication(BusinessApplicationEntity),
    PostApiGatewayInfoEntity(ApiGatewayInfoEntity),
    PostForwardProxyEntity(ForwardProxyEntity),
    PostProfile(ApplicationProfileEntity),
    PostCertificate(CertificateEntity),
    PostApiClientCredential(ApiCredentialsEntity),
    PostSag(SagEntity),
    DeleteApiGatewayInfoEntity(ApiGatewayInfoEntity),
    DeleteForwardProxyEntity(ForwardProxyEntity),
    DeleteBusinessApplication(BusinessApplicationEntity),
    DeleteCertificate(CertificateEntity),
    DeleteSag(SagEntity),
    DeleteProfile(ApplicationProfileEntity),
}
