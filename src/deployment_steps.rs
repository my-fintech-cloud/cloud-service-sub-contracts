use serde::{Deserialize, Serialize};
service_sdk::macros::use_my_sb_entity_protobuf_model!();

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct DeploymentStepMetadata {
    #[prost(string, tag = "1")]
    pub instance_id: String,
    #[prost(string, tag = "2")]
    pub product_id: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
pub enum DeploymentStepSbType {
    InfrastructureSetup = 0,
    InfrastructureServicesSetup = 1,
    ServicesDeployment = 2,
    NginxConfiguration = 3,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "deployment-step-execute")]
#[derive(Serialize, Deserialize)]
pub struct DeploymentStepExecuteSbCommand {
    #[prost(message, tag = "1")]
    pub metadata: Option<DeploymentStepMetadata>,
    #[prost(message, tag = "2")]
    pub step: Option<DeploymentStepSbModel>,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct DeploymentStepSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub instance_id: String,
    #[prost(string, tag = "3")]
    pub product_id: String,
    #[prost(enumeration = "DeploymentStepSbType", tag = "4")]
    pub step_type: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(Serialize, Deserialize)]
pub enum DeploymentStepExecutedResult {
    Success = 0,
    Failed = 1,
}

#[derive(Clone, PartialEq, ::prost::Message)]
#[my_sb_entity_protobuf_model(topic_id = "deployment-step-executed")]
#[derive(Serialize, Deserialize)]
pub struct DeploymentStepExecutedSbModel {
    #[prost(message, tag = "1")]
    pub step: Option<DeploymentStepSbModel>,
    #[prost(enumeration = "DeploymentStepExecutedResult", tag = "2")]
    pub result: i32,
}
