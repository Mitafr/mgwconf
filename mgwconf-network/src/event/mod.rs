use mgw_configuration::models::{ApiGatewayInfoEntity, ApplicationProfileEntity, BusinessApplicationEntity, CertificateEntity, ForwardProxyEntity, SagEntity};

#[derive(Debug, Clone)]
pub enum IoEvent {
    Ping,
    GetAllApiGatewayInfoEntity,
    GetAllForwardProxyEntity,
    GetAllBusinessApplications,
    GetAllCertificates,
    GetAllSags,
    GetAllProfiles,
    PostBusinessApplication(BusinessApplicationEntity),
    PostApiGatewayInfoEntity(ApiGatewayInfoEntity),
    PostForwardProxyEntity(ForwardProxyEntity),
    PostProfile(ApplicationProfileEntity),
    PostCertificate(CertificateEntity),
    PostSag(SagEntity),
    DeleteApiGatewayInfoEntity(ApiGatewayInfoEntity),
    DeleteForwardProxyEntity(ForwardProxyEntity),
    DeleteBusinessApplication(BusinessApplicationEntity),
    DeleteCertificate(CertificateEntity),
    DeleteSag(SagEntity),
    DeleteProfile(ApplicationProfileEntity),
}
