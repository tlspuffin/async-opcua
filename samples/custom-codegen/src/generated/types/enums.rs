// This file was autogenerated from schema/Opc.Ua.Pn.Types.bsd by async-opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Einar Omang
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum IMTagSelectorEnumeration {
    #[opcua(default)]
    FUNCTION = 0i32,
    LOCATION = 1i32,
    BOTH = 2i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnARStateEnumeration {
    #[opcua(default)]
    CONNECTED = 0i32,
    UNCONNECTED = 1i32,
    UNCONNECTED_ERR_DEVICE_NOT_FOUND = 2i32,
    UNCONNECTED_ERR_DUPLICATE_IP = 3i32,
    UNCONNECTED_ERR_DUPLICATE_NOS = 4i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnARTypeEnumeration {
    #[opcua(default)]
    IOCARSingle = 0i32,
    IOSAR = 6i32,
    IOCARSingleUsingRT_CLASS_3 = 16i32,
    IOCARSR = 32i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnAssetChangeEnumeration {
    #[opcua(default)]
    INSERTED = 0i32,
    REMOVED = 1i32,
    CHANGED = 2i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnAssetTypeEnumeration {
    #[opcua(default)]
    DEVICE = 0i32,
    MODULE = 1i32,
    SUBMODULE = 2i32,
    ASSET = 3i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnChannelAccumulativeEnumeration {
    #[opcua(default)]
    SINGLE = 0i32,
    ACCUMULATIVE = 256i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnChannelDirectionEnumeration {
    #[opcua(default)]
    MANUFACTURER_SPECIFIC = 0i32,
    INPUT_CHANNEL = 8192i32,
    OUTPUT_CHANNEL = 16384i32,
    BIDIRECTIONAL_CHANNEL = 24576i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnChannelMaintenanceEnumeration {
    #[opcua(default)]
    FAULT = 0i32,
    MAINTENANCE_REQUIRED = 512i32,
    MAINTENANCE_DEMANDED = 1024i32,
    USE_QUALIFIED_CHANNEL_QUALIFIER = 1536i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnChannelSpecifierEnumeration {
    #[opcua(default)]
    ALL_DISAPPEARS = 0i32,
    APPEARS = 2048i32,
    DISAPPEARS = 4096i32,
    DISAPPEARS_OTHER_REMAIN = 6144i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnChannelTypeEnumeration {
    #[opcua(default)]
    UNSPECIFIC = 0i32,
    __1BIT = 1i32,
    __2BIT = 2i32,
    __4BIT = 3i32,
    __8BIT = 4i32,
    __16BIT = 5i32,
    __32BIT = 6i32,
    __64BIT = 7i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnDeviceStateEnumeration {
    #[opcua(default)]
    OFFLINE = 0i32,
    OFFLINE_DOCKING = 1i32,
    ONLINE = 2i32,
    ONLINE_DOCKING = 3i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnLinkStateEnumeration {
    UP = 1i32,
    DOWN = 2i32,
    TESTING = 3i32,
    UNKNOWN = 4i32,
    DORMANT = 5i32,
    NOT_PRESENT = 6i32,
    LOWER_LAYER_DOWN = 7i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnModuleStateEnumeration {
    #[opcua(default)]
    NO_MODULE = 0i32,
    WRONG_MODULE = 1i32,
    PROPER_MODULE = 2i32,
    SUBSTITUTE = 3i32,
    OK = 4i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnPortStateEnumeration {
    #[opcua(default)]
    UNKNOWN = 0i32,
    DISABLED_DISCARDING = 1i32,
    BLOCKING = 2i32,
    LISTENING = 3i32,
    LEARNING = 4i32,
    FORWARDING = 5i32,
    BROKEN = 6i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnSubmoduleAddInfoEnumeration {
    #[opcua(default)]
    NO_ADD_INFO = 0i32,
    TAKEOVER_NOT_ALLOWED = 1i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnSubmoduleARInfoEnumeration {
    #[opcua(default)]
    OWN = 0i32,
    APPLICATION_READY_PENDING = 128i32,
    SUPERORDINATED_LOCKED = 256i32,
    LOCKED_BY_IO_CONTROLLER = 384i32,
    LOCKED_BY_IO_SUPERVISOR = 512i32,
}
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    opcua::types::UaEnum,
    opcua::types::BinaryEncodable,
    opcua::types::BinaryDecodable,
)]
#[cfg_attr(
    feature = "json",
    derive(opcua::types::JsonEncodable, opcua::types::JsonDecodable)
)]
#[cfg_attr(
    feature = "xml",
    derive(
        opcua::types::XmlEncodable,
        opcua::types::XmlDecodable,
        opcua::types::XmlType
    )
)]
#[repr(i32)]
pub enum PnSubmoduleIdentInfoEnumeration {
    #[opcua(default)]
    OK = 0i32,
    SUBSTITUTE = 2048i32,
    WRONG = 4096i32,
    NO_SUBMODULE = 6144i32,
}
