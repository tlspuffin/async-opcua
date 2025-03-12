// This file was autogenerated from schema/Opc.Ua.Pn.NodeSet2.xml by async-opcua-codegen
//
// DO NOT EDIT THIS FILE

// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Einar Omang
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.17
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum IMTagSelectorEnumeration {
    #[opcua(default)]
    FUNCTION = 0i32,
    LOCATION = 1i32,
    BOTH = 2i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.2
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnARStateEnumeration {
    #[opcua(default)]
    ///The AR connection to the device is established
    CONNECTED = 0i32,
    ///The AR connection to the device is not established
    UNCONNECTED = 1i32,
    ///The AR connection to the device is not established because the device is not available in the network
    UNCONNECTED_ERR_DEVICE_NOT_FOUND = 2i32,
    ///The AR connection to the device is not established because the IP address of the device exists multiple times
    UNCONNECTED_ERR_DUPLICATE_IP = 3i32,
    ///The AR connection to the device is not established because the Name of Station of the device exists multiple times
    UNCONNECTED_ERR_DUPLICATE_NOS = 4i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.3
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnARTypeEnumeration {
    #[opcua(default)]
    IOCARSingle = 0i32,
    ///The supervisor AR is a special form of the IOCARSingle allowing takeover of the ownership of a submodule
    IOSAR = 6i32,
    ///This is a special form of the IOCARSingle indicating RT_CLASS_3 communication
    IOCARSingleUsingRT_CLASS_3 = 16i32,
    ///The SR AR is a special form of the IOCARSingle indicating system redundancy or dynamic reconfiguration usage
    IOCARSR = 32i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.14
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnAssetChangeEnumeration {
    #[opcua(default)]
    ///Asset has been added
    INSERTED = 0i32,
    ///Asset has been removed
    REMOVED = 1i32,
    ///Asset has been changed
    CHANGED = 2i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.13
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnAssetTypeEnumeration {
    #[opcua(default)]
    ///Device
    DEVICE = 0i32,
    ///Real Module
    MODULE = 1i32,
    ///Real Submodule
    SUBMODULE = 2i32,
    ///Asset
    ASSET = 3i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.9
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnChannelAccumulativeEnumeration {
    #[opcua(default)]
    /**Single channel
    Diagnosis only for the reported channel
    */
    SINGLE = 0i32,
    /**Multiple channel
    Accumulative diagnosis from more than one channel
    */
    ACCUMULATIVE = 256i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.12
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnChannelDirectionEnumeration {
    #[opcua(default)]
    ///Manufacturer specific
    MANUFACTURER_SPECIFIC = 0i32,
    ///Input
    INPUT_CHANNEL = 8192i32,
    ///Output
    OUTPUT_CHANNEL = 16384i32,
    ///Input/Output
    BIDIRECTIONAL_CHANNEL = 24576i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.10
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnChannelMaintenanceEnumeration {
    #[opcua(default)]
    ///Fault
    FAULT = 0i32,
    ///Maintenance required
    MAINTENANCE_REQUIRED = 512i32,
    ///Maintenance demanded
    MAINTENANCE_DEMANDED = 1024i32,
    ///Use QualifiedChannelQualifier variable
    USE_QUALIFIED_CHANNEL_QUALIFIER = 1536i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.11
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnChannelSpecifierEnumeration {
    #[opcua(default)]
    ///The Diagnosis ASE contains no longer any entries (of any severity) for this channel
    ALL_DISAPPEARS = 0i32,
    /**An event appears and/or exists further
    The Diagnosis ASE contains this and possible other entries for this channel.
    */
    APPEARS = 2048i32,
    /**An event disappears and/or exists no longer
    The Diagnosis ASE contains no longer any entries of the same severity for this channel
    */
    DISAPPEARS = 4096i32,
    /**An event disappears
    The Diagnosis ASE still contains other entries of the same severity for this channel
    */
    DISAPPEARS_OTHER_REMAIN = 6144i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.8
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnChannelTypeEnumeration {
    #[opcua(default)]
    /**Shall be used if the field ChannelNumber contains the value 0x8000 (submodule)
    Furthermore, it shall be used if none of the below defined types are appropriate.
    */
    UNSPECIFIC = 0i32,
    ///The data length of this channel is 1 Bit.
    #[opcua(rename = "1BIT")]
    __1BIT = 1i32,
    ///The data length of this channel is 2 Bit.
    #[opcua(rename = "2BIT")]
    __2BIT = 2i32,
    ///The data length of this channel is 4 Bit.
    #[opcua(rename = "4BIT")]
    __4BIT = 3i32,
    ///The data length of this channel is 8 Bit.
    #[opcua(rename = "8BIT")]
    __8BIT = 4i32,
    ///The data length of this channel is 16 Bit.
    #[opcua(rename = "16BIT")]
    __16BIT = 5i32,
    ///The data length of this channel is 32 Bit.
    #[opcua(rename = "32BIT")]
    __32BIT = 6i32,
    ///The data length of this channel is 64 Bit.
    #[opcua(rename = "64BIT")]
    __64BIT = 7i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.1
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnDeviceStateEnumeration {
    #[opcua(default)]
    ///The device is not online, or no information is available. The device is offline if no ARs other than possible Device Access AR’s exist.
    OFFLINE = 0i32,
    ///The device is a docking device and currently not online.
    OFFLINE_DOCKING = 1i32,
    ///The device is online. This is the case if at least one AR other than possible Device Access AR’s exists.
    ONLINE = 2i32,
    ///The device is a docking device and currently online.
    ONLINE_DOCKING = 3i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.15
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnLinkStateEnumeration {
    ///Ready to pass packets
    UP = 1i32,
    ///No packets are passed
    DOWN = 2i32,
    ///In some test mode
    TESTING = 3i32,
    ///Status cannot be determined
    UNKNOWN = 4i32,
    ///In pending state waiting  for some external event
    DORMANT = 5i32,
    ///Port not present
    NOT_PRESENT = 6i32,
    ///Down due to lower layer
    LOWER_LAYER_DOWN = 7i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.4
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnModuleStateEnumeration {
    #[opcua(default)]
    ///For example module not plugged
    NO_MODULE = 0i32,
    ///For example ModuleIdentNumber wrong
    WRONG_MODULE = 1i32,
    ///Module is okay but at least one submodule is locked, wrong or missing
    PROPER_MODULE = 2i32,
    ///Module is not the same as requested – but the IO device was able to adapt by its own knowledge
    SUBSTITUTE = 3i32,
    ///Default state
    OK = 4i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.16
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnPortStateEnumeration {
    #[opcua(default)]
    ///Status cannot be determined
    UNKNOWN = 0i32,
    ///The port is administratively disabled and discarding frames
    DISABLED_DISCARDING = 1i32,
    ///The port blocks incoming frames
    BLOCKING = 2i32,
    ///The port is listening to and sending BPDUs (Bridge Protocol Data Units).
    LISTENING = 3i32,
    LEARNING = 4i32,
    FORWARDING = 5i32,
    BROKEN = 6i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.5
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnSubmoduleAddInfoEnumeration {
    #[opcua(default)]
    NO_ADD_INFO = 0i32,
    ///This Submodule is not available for takeover by IOSAR.
    TAKEOVER_NOT_ALLOWED = 1i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.6
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnSubmoduleARInfoEnumeration {
    #[opcua(default)]
    ///This AR is owner of the submodule
    OWN = 0i32,
    ///This AR is owner of the submodule but it is blocked. For example parameter checking pending
    APPLICATION_READY_PENDING = 128i32,
    ///This AR is not owner of the submodule. It is blocked by superordinated means
    SUPERORDINATED_LOCKED = 256i32,
    ///This AR is not owner of the submodule. It is owned by another IOAR
    LOCKED_BY_IO_CONTROLLER = 384i32,
    ///This AR is not owner of the submodule. It is owned by another IOSAR
    LOCKED_BY_IO_SUPERVISOR = 512i32,
}
#[opcua::types::ua_encodable]
///https://reference.opcfoundation.org/v104/PROFINET/v101/docs/6.3.3/#6.3.3.3.7
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PnSubmoduleIdentInfoEnumeration {
    #[opcua(default)]
    ///OK
    OK = 0i32,
    ///Substitute (SU)
    SUBSTITUTE = 2048i32,
    ///Wrong (WR)
    WRONG = 4096i32,
    ///NoSubmodule (NO)
    NO_SUBMODULE = 6144i32,
}
