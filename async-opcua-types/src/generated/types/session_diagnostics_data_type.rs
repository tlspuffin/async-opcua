// This file was autogenerated from schemas/1.05/Opc.Ua.NodeSet2.Services.xml by async-opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
#[allow(unused)]
mod opcua {
    pub use crate as types;
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v105/Core/docs/Part5/12.11
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SessionDiagnosticsDataType {
    pub session_id: opcua::types::node_id::NodeId,
    pub session_name: opcua::types::string::UAString,
    pub client_description: super::application_description::ApplicationDescription,
    pub server_uri: opcua::types::string::UAString,
    pub endpoint_url: opcua::types::string::UAString,
    pub locale_ids: Option<Vec<opcua::types::LocaleId>>,
    pub actual_session_timeout: opcua::types::data_types::Duration,
    pub max_response_message_size: u32,
    pub client_connection_time: opcua::types::data_types::UtcTime,
    pub client_last_contact_time: opcua::types::data_types::UtcTime,
    pub current_subscriptions_count: u32,
    pub current_monitored_items_count: u32,
    pub current_publish_requests_in_queue: u32,
    pub total_request_count: super::service_counter_data_type::ServiceCounterDataType,
    pub unauthorized_request_count: u32,
    pub read_count: super::service_counter_data_type::ServiceCounterDataType,
    pub history_read_count: super::service_counter_data_type::ServiceCounterDataType,
    pub write_count: super::service_counter_data_type::ServiceCounterDataType,
    pub history_update_count: super::service_counter_data_type::ServiceCounterDataType,
    pub call_count: super::service_counter_data_type::ServiceCounterDataType,
    pub create_monitored_items_count: super::service_counter_data_type::ServiceCounterDataType,
    pub modify_monitored_items_count: super::service_counter_data_type::ServiceCounterDataType,
    pub set_monitoring_mode_count: super::service_counter_data_type::ServiceCounterDataType,
    pub set_triggering_count: super::service_counter_data_type::ServiceCounterDataType,
    pub delete_monitored_items_count: super::service_counter_data_type::ServiceCounterDataType,
    pub create_subscription_count: super::service_counter_data_type::ServiceCounterDataType,
    pub modify_subscription_count: super::service_counter_data_type::ServiceCounterDataType,
    pub set_publishing_mode_count: super::service_counter_data_type::ServiceCounterDataType,
    pub publish_count: super::service_counter_data_type::ServiceCounterDataType,
    pub republish_count: super::service_counter_data_type::ServiceCounterDataType,
    pub transfer_subscriptions_count: super::service_counter_data_type::ServiceCounterDataType,
    pub delete_subscriptions_count: super::service_counter_data_type::ServiceCounterDataType,
    pub add_nodes_count: super::service_counter_data_type::ServiceCounterDataType,
    pub add_references_count: super::service_counter_data_type::ServiceCounterDataType,
    pub delete_nodes_count: super::service_counter_data_type::ServiceCounterDataType,
    pub delete_references_count: super::service_counter_data_type::ServiceCounterDataType,
    pub browse_count: super::service_counter_data_type::ServiceCounterDataType,
    pub browse_next_count: super::service_counter_data_type::ServiceCounterDataType,
    pub translate_browse_paths_to_node_ids_count:
        super::service_counter_data_type::ServiceCounterDataType,
    pub query_first_count: super::service_counter_data_type::ServiceCounterDataType,
    pub query_next_count: super::service_counter_data_type::ServiceCounterDataType,
    pub register_nodes_count: super::service_counter_data_type::ServiceCounterDataType,
    pub unregister_nodes_count: super::service_counter_data_type::ServiceCounterDataType,
}
impl opcua::types::MessageInfo for SessionDiagnosticsDataType {
    fn type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionDiagnosticsDataType_Encoding_DefaultBinary
    }
    fn json_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionDiagnosticsDataType_Encoding_DefaultJson
    }
    fn xml_type_id(&self) -> opcua::types::ObjectId {
        opcua::types::ObjectId::SessionDiagnosticsDataType_Encoding_DefaultXml
    }
    fn data_type_id(&self) -> opcua::types::DataTypeId {
        opcua::types::DataTypeId::SessionDiagnosticsDataType
    }
}
