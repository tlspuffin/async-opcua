// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock, Einar Omang
//
// This file was autogenerated from tools/schema/schemas/1.0.4/Opc.Ua.Types.bsd by opcua-codegen
//
// DO NOT EDIT THIS FILE
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PublishResponse {
    pub response_header: crate::types::response_header::ResponseHeader,
    pub subscription_id: u32,
    pub available_sequence_numbers: Option<Vec<u32>>,
    pub more_notifications: bool,
    pub notification_message: super::notification_message::NotificationMessage,
    pub results: Option<Vec<crate::types::status_code::StatusCode>>,
    pub diagnostic_infos: Option<Vec<crate::types::diagnostic_info::DiagnosticInfo>>,
}
impl crate::types::MessageInfo for PublishResponse {
    fn object_id(&self) -> crate::types::ObjectId {
        crate::types::ObjectId::PublishResponse_Encoding_DefaultBinary
    }
}
impl crate::types::BinaryEncoder<PublishResponse> for PublishResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0usize;
        size += self.response_header.byte_len();
        size += self.subscription_id.byte_len();
        size += self.available_sequence_numbers.byte_len();
        size += self.more_notifications.byte_len();
        size += self.notification_message.byte_len();
        size += self.results.byte_len();
        size += self.diagnostic_infos.byte_len();
        size
    }
    #[allow(unused_variables)]
    fn encode<S: std::io::Write>(&self, stream: &mut S) -> crate::types::EncodingResult<usize> {
        let mut size = 0usize;
        size += self.response_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.available_sequence_numbers.encode(stream)?;
        size += self.more_notifications.encode(stream)?;
        size += self.notification_message.encode(stream)?;
        size += self.results.encode(stream)?;
        size += self.diagnostic_infos.encode(stream)?;
        Ok(size)
    }
    #[allow(unused_variables)]
    fn decode<S: std::io::Read>(
        stream: &mut S,
        decoding_options: &crate::types::DecodingOptions,
    ) -> crate::types::EncodingResult<Self> {
        let response_header =
            <crate::types::response_header::ResponseHeader as crate::types::BinaryEncoder<
                crate::types::response_header::ResponseHeader,
            >>::decode(stream, decoding_options)?;
        let subscription_id =
            <u32 as crate::types::BinaryEncoder<u32>>::decode(stream, decoding_options)?;
        let available_sequence_numbers = <Option<Vec<u32>> as crate::types::BinaryEncoder<
            Option<Vec<u32>>,
        >>::decode(stream, decoding_options)?;
        let more_notifications =
            <bool as crate::types::BinaryEncoder<bool>>::decode(stream, decoding_options)?;
        let notification_message =
            <super::notification_message::NotificationMessage as crate::types::BinaryEncoder<
                super::notification_message::NotificationMessage,
            >>::decode(stream, decoding_options)?;
        let results =
            <Option<Vec<crate::types::status_code::StatusCode>> as crate::types::BinaryEncoder<
                Option<Vec<crate::types::status_code::StatusCode>>,
            >>::decode(stream, decoding_options)?;
        let diagnostic_infos = <Option<
            Vec<crate::types::diagnostic_info::DiagnosticInfo>,
        > as crate::types::BinaryEncoder<
            Option<Vec<crate::types::diagnostic_info::DiagnosticInfo>>,
        >>::decode(stream, decoding_options)?;
        Ok(Self {
            response_header,
            subscription_id,
            available_sequence_numbers,
            more_notifications,
            notification_message,
            results,
            diagnostic_infos,
        })
    }
}
