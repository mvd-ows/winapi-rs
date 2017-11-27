// Copyright © 2016-2017 winapi-rs developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! WFP Structures and Definitions
use ctypes::{c_double, c_float, wchar_t};
use shared::basetsd::{UINT8, UINT16, UINT32, UINT64, INT8, INT16, INT32, INT64};
use shared::minwindef::{DWORD, ULONG};
use um::winnt::{LPWSTR, PSID_AND_ATTRIBUTES, SID};

pub use shared::ntdef::LUID;

ENUM!{enum FWP_DIRECTION {
    FWP_DIRECTION_OUTBOUND = 0,
    FWP_DIRECTION_INBOUND = FWP_DIRECTION_OUTBOUND + 1,
    FWP_DIRECTION_MAX = FWP_DIRECTION_INBOUND + 1,
}}

ENUM!{enum FWP_IP_VERSION {
    FWP_IP_VERSION_V4 = 0,
    FWP_IP_VERSION_V6 = FWP_IP_VERSION_V4 + 1,
    FWP_IP_VERSION_NONE = FWP_IP_VERSION_V6 + 1,
    FWP_IP_VERSION_MAX = FWP_IP_VERSION_NONE + 1,
}}

ENUM!{enum FWP_AF {
    FWP_AF_INET = FWP_IP_VERSION_V4,
    FWP_AF_INET6 = FWP_IP_VERSION_V6,
    FWP_AF_ETHER = FWP_IP_VERSION_NONE,
    FWP_AF_NONE = FWP_AF_ETHER + 1,
}}

ENUM!{enum FWP_ETHER_ENCAP_METHOD {
    FWP_ETHER_ENCAP_METHOD_ETHER_V2 = 0,
    FWP_ETHER_ENCAP_METHOD_SNAP = 1,
    FWP_ETHER_ENCAP_METHOD_SNAP_W_OUI_ZERO = 3,
}}

ENUM!{enum FWP_DATA_TYPE {
    FWP_EMPTY = 0,
    FWP_UINT8 = FWP_EMPTY + 1,
    FWP_UINT16 = FWP_UINT8 + 1,
    FWP_UINT32 = FWP_UINT16 + 1,
    FWP_UINT64 = FWP_UINT32 + 1,
    FWP_INT8 = FWP_UINT64 + 1,
    FWP_INT16 = FWP_INT8 + 1,
    FWP_INT32 = FWP_INT16 + 1,
    FWP_INT64 = FWP_INT32 + 1,
    FWP_FLOAT = FWP_INT64 + 1,
    FWP_DOUBLE = FWP_FLOAT + 1,
    FWP_BYTE_ARRAY16_TYPE = FWP_DOUBLE + 1,
    FWP_BYTE_BLOB_TYPE = FWP_BYTE_ARRAY16_TYPE + 1,
    FWP_SID = FWP_BYTE_BLOB_TYPE + 1,
    FWP_SECURITY_DESCRIPTOR_TYPE = FWP_SID + 1,
    FWP_TOKEN_INFORMATION_TYPE = FWP_SECURITY_DESCRIPTOR_TYPE + 1,
    FWP_TOKEN_ACCESS_INFORMATION_TYPE = FWP_TOKEN_INFORMATION_TYPE + 1,
    FWP_UNICODE_STRING_TYPE = FWP_TOKEN_ACCESS_INFORMATION_TYPE + 1,
    FWP_BYTE_ARRAY6_TYPE = FWP_UNICODE_STRING_TYPE + 1,
    FWP_BITMAP_INDEX_TYPE = FWP_BYTE_ARRAY6_TYPE + 1,
    FWP_BITMAP_ARRAY64_TYPE = FWP_BITMAP_INDEX_TYPE + 1,
    FWP_SINGLE_DATA_TYPE_MAX = 0xff,
    FWP_V4_ADDR_MASK = FWP_SINGLE_DATA_TYPE_MAX + 1,
    FWP_V6_ADDR_MASK = FWP_V4_ADDR_MASK + 1,
    FWP_RANGE_TYPE = FWP_V6_ADDR_MASK + 1,
    FWP_DATA_TYPE_MAX = FWP_RANGE_TYPE + 1,
}}

STRUCT!{struct FWP_BITMAP_ARRAY64 {
    bitmapArray64: [UINT8; 8],
}}

pub const FWP_BYTEMAP_ARRAY64_SIZE: usize = 8;
pub const FWP_BITMAP_ARRAY64_SIZE: usize = 64;

STRUCT!{struct FWP_BYTE_ARRAY6 {
    byteArray6: [UINT8; 6],
}}

pub const FWP_BYTE_ARRAY6_SIZE: usize = 6;

STRUCT!{struct FWP_BYTE_ARRAY16 {
    byteArray16: [UINT8; 16],
}}

STRUCT!{struct FWP_BYTE_BLOB {
    size: UINT32,
    data: *mut UINT8,
}}

STRUCT!{struct FWP_TOKEN_INFORMATION {
    sidCount: ULONG,
    sids: PSID_AND_ATTRIBUTES,
    restrictedSidCount: ULONG,
    restrictedSids: PSID_AND_ATTRIBUTES,
}}

UNION!{union FWP_VALUE0_u {
    [u32; 1] [u64; 1],
    uint8 uint8_mut: UINT8,
    uint16 uint16_mut: UINT16,
    uint32 uint32_mut: UINT32,
    uint64 uint64_mut: *mut UINT64,
    int8 int8_mut: INT8,
    int16 int16_mut: INT16,
    int32 int32_mut: INT32,
    int64 int64_mut: *mut INT64,
    float32 float32_mut: c_float,
    double64 double64_mut: *mut c_double,
    byteArray16 byteArray16_mut: *mut FWP_BYTE_ARRAY16,
    byteBlob byteBlob_mut: *mut FWP_BYTE_BLOB,
    sid sid_mut: *mut SID,
    sd sd_mut: *mut FWP_BYTE_BLOB,
    tokenInformation tokenInformation_mut: *mut FWP_TOKEN_INFORMATION,
    tokenAccessInformation tokenAccessInformation_mut: *mut FWP_BYTE_BLOB,
    unicodeString unicodeString_mut: LPWSTR,
    byteArray6 byteArray6_mut: *mut FWP_BYTE_ARRAY6,
    bitmapArray64 bitmapArray64_mut: *mut FWP_BITMAP_ARRAY64,
}}

STRUCT!{struct FWP_VALUE0 {
    type_: FWP_DATA_TYPE,
    u: FWP_VALUE0_u,
}}

ENUM!{enum FWP_MATCH_TYPE {
    FWP_MATCH_EQUAL = 0,
    FWP_MATCH_GREATER = FWP_MATCH_EQUAL + 1,
    FWP_MATCH_LESS = FWP_MATCH_GREATER + 1,
    FWP_MATCH_GREATER_OR_EQUAL = FWP_MATCH_LESS + 1,
    FWP_MATCH_LESS_OR_EQUAL = FWP_MATCH_GREATER_OR_EQUAL + 1,
    FWP_MATCH_RANGE = FWP_MATCH_LESS_OR_EQUAL + 1,
    FWP_MATCH_FLAGS_ALL_SET = FWP_MATCH_RANGE + 1,
    FWP_MATCH_FLAGS_ANY_SET = FWP_MATCH_FLAGS_ALL_SET + 1,
    FWP_MATCH_FLAGS_NONE_SET = FWP_MATCH_FLAGS_ANY_SET + 1,
    FWP_MATCH_EQUAL_CASE_INSENSITIVE = FWP_MATCH_FLAGS_NONE_SET + 1,
    FWP_MATCH_NOT_EQUAL = FWP_MATCH_EQUAL_CASE_INSENSITIVE + 1,
    FWP_MATCH_PREFIX = FWP_MATCH_NOT_EQUAL + 1,
    FWP_MATCH_NOT_PREFIX = FWP_MATCH_PREFIX + 1,
    FWP_MATCH_TYPE_MAX = FWP_MATCH_NOT_PREFIX + 1,
}}

STRUCT!{struct FWP_V4_ADDR_AND_MASK {
    addr: UINT32,
    mask: UINT32,
}}

pub const FWP_V6_ADDR_SIZE: usize = 16;

STRUCT!{struct FWP_V6_ADDR_AND_MASK {
    addr: [UINT8; 16],
    prefixLength: UINT8,
}}

STRUCT!{struct FWP_RANGE0 {
    valueLow: FWP_VALUE0,
    valueHigh: FWP_VALUE0,
}}

pub const FWP_ACTRL_MATCH_FILTER: DWORD = 0x00000001;

UNION!{union FWP_CONDITION_VALUE0_u {
    [u32; 1] [u64; 1],
    uint8 uint8_mut: UINT8,
    uint16 uint16_mut: UINT16,
    uint32 uint32_mut: UINT32,
    uint64 uint64_mut: *mut UINT64,
    int8 int8_mut: INT8,
    int16 int16_mut: INT16,
    int32 int32_mut: INT32,
    int64 int64_mut: *mut INT64,
    float32 float32_mut: c_float,
    double64 double64_mut: *mut c_double,
    byteArray16 byteArray16_mut: *mut FWP_BYTE_ARRAY16,
    byteBlob byteBlob_mut: *mut FWP_BYTE_BLOB,
    sid sid_mut: *mut SID,
    sd sd_mut: *mut FWP_BYTE_BLOB,
    tokenInformation tokenInformation_mut: *mut FWP_TOKEN_INFORMATION,
    tokenAccessInformation tokenAccessInformation_mut: *mut FWP_BYTE_BLOB,
    unicodeString unicodeString_mut: LPWSTR,
    byteArray6 byteArray6_mut: *mut FWP_BYTE_ARRAY6,
    bitmapArray64 bitmapArray64_mut: *mut FWP_BITMAP_ARRAY64,
    v4AddrMask v4AddrMask_mut: *mut FWP_V4_ADDR_AND_MASK,
    v6AddrMask v6AddrMask_mut: *mut FWP_V6_ADDR_AND_MASK,
    rangeValue rangeValue_mut: *mut FWP_RANGE0,
}}

STRUCT!{struct FWP_CONDITION_VALUE0 {
    type_: FWP_DATA_TYPE,
    u: FWP_CONDITION_VALUE0_u,
}}

ENUM!{enum FWP_CLASSIFY_OPTION_TYPE {
    FWP_CLASSIFY_OPTION_MULTICAST_STATE = 0,
    FWP_CLASSIFY_OPTION_LOOSE_SOURCE_MAPPING = FWP_CLASSIFY_OPTION_MULTICAST_STATE + 1,
    FWP_CLASSIFY_OPTION_UNICAST_LIFETIME = FWP_CLASSIFY_OPTION_LOOSE_SOURCE_MAPPING + 1,
    FWP_CLASSIFY_OPTION_MCAST_BCAST_LIFETIME = FWP_CLASSIFY_OPTION_UNICAST_LIFETIME + 1,
    FWP_CLASSIFY_OPTION_SECURE_SOCKET_SECURITY_FLAGS =
        FWP_CLASSIFY_OPTION_MCAST_BCAST_LIFETIME + 1,
    FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_MM_POLICY_KEY =
        FWP_CLASSIFY_OPTION_SECURE_SOCKET_SECURITY_FLAGS + 1,
    FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_QM_POLICY_KEY =
        FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_MM_POLICY_KEY + 1,
    FWP_CLASSIFY_OPTION_LOCAL_ONLY_MAPPING =
        FWP_CLASSIFY_OPTION_SECURE_SOCKET_AUTHIP_QM_POLICY_KEY + 1,
    FWP_CLASSIFY_OPTION_MAX = FWP_CLASSIFY_OPTION_LOCAL_ONLY_MAPPING + 1,
}}

pub const FWP_OPTION_VALUE_ALLOW_MULTICAST_STATE: DWORD = 0x00000000;
pub const FWP_OPTION_VALUE_DENY_MULTICAST_STATE: DWORD = 0x00000001;
pub const FWP_OPTION_VALUE_ALLOW_GLOBAL_MULTICAST_STATE: DWORD = 0x00000002;
pub const FWP_OPTION_VALUE_DISABLE_LOOSE_SOURCE: DWORD = 0x00000000;
pub const FWP_OPTION_VALUE_ENABLE_LOOSE_SOURCE: DWORD = 0x00000001;
pub const FWP_OPTION_VALUE_DISABLE_LOCAL_ONLY_MAPPING: DWORD = 0x00000000;
pub const FWP_OPTION_VALUE_ENABLE_LOCAL_ONLY_MAPPING: DWORD = 0x00000001;

ENUM!{enum FWP_VSWITCH_NETWORK_TYPE {
    FWP_VSWITCH_NETWORK_TYPE_UNKNOWN = 0,
    FWP_VSWITCH_NETWORK_TYPE_PRIVATE = FWP_VSWITCH_NETWORK_TYPE_UNKNOWN + 1,
    FWP_VSWITCH_NETWORK_TYPE_INTERNAL = FWP_VSWITCH_NETWORK_TYPE_PRIVATE + 1,
    FWP_VSWITCH_NETWORK_TYPE_EXTERNAL = FWP_VSWITCH_NETWORK_TYPE_INTERNAL + 1,
}}

pub const FWP_ACTION_FLAG_TERMINATING: DWORD = 0x00001000;
pub const FWP_ACTION_FLAG_NON_TERMINATING: DWORD = 0x00002000;
pub const FWP_ACTION_FLAG_CALLOUT: DWORD = 0x00004000;

pub type FWP_ACTION_TYPE = UINT32;

pub const FWP_ACTION_BLOCK: DWORD = 0x00000001 | FWP_ACTION_FLAG_TERMINATING;
pub const FWP_ACTION_PERMIT: DWORD = 0x00000002 | FWP_ACTION_FLAG_TERMINATING;
pub const FWP_ACTION_CALLOUT_TERMINATING: DWORD =
    0x00000003 | FWP_ACTION_FLAG_CALLOUT | FWP_ACTION_FLAG_TERMINATING;
pub const FWP_ACTION_CALLOUT_INSPECTION: DWORD =
    0x00000004 | FWP_ACTION_FLAG_CALLOUT | FWP_ACTION_FLAG_NON_TERMINATING;
pub const FWP_ACTION_CALLOUT_UNKNOWN: DWORD = 0x00000005 | FWP_ACTION_FLAG_CALLOUT;
pub const FWP_ACTION_CONTINUE: DWORD = 0x00000006 | FWP_ACTION_FLAG_NON_TERMINATING;
pub const FWP_ACTION_NONE: DWORD = 0x00000007;
pub const FWP_ACTION_NONE_NO_MATCH: DWORD = 0x00000008;
pub const FWP_ACTION_BITMAP_INDEX_SET: DWORD = 0x00000009;
pub const FWP_CONDITION_FLAG_IS_LOOPBACK: DWORD = 0x00000001;
pub const FWP_CONDITION_FLAG_IS_IPSEC_SECURED: DWORD = 0x00000002;
pub const FWP_CONDITION_FLAG_IS_REAUTHORIZE: DWORD = 0x00000004;
pub const FWP_CONDITION_FLAG_IS_WILDCARD_BIND: DWORD = 0x00000008;
pub const FWP_CONDITION_FLAG_IS_RAW_ENDPOINT: DWORD = 0x00000010;
pub const FWP_CONDITION_FLAG_IS_FRAGMENT: DWORD = 0x00000020;
pub const FWP_CONDITION_FLAG_IS_FRAGMENT_GROUP: DWORD = 0x00000040;
pub const FWP_CONDITION_FLAG_IS_IPSEC_NATT_RECLASSIFY: DWORD = 0x00000080;
pub const FWP_CONDITION_FLAG_REQUIRES_ALE_CLASSIFY: DWORD = 0x00000100;
pub const FWP_CONDITION_FLAG_IS_IMPLICIT_BIND: DWORD = 0x00000200;
pub const FWP_CONDITION_FLAG_IS_REASSEMBLED: DWORD = 0x00000400;
pub const FWP_CONDITION_FLAG_IS_NAME_APP_SPECIFIED: DWORD = 0x00004000;
pub const FWP_CONDITION_FLAG_IS_PROMISCUOUS: DWORD = 0x00008000;
pub const FWP_CONDITION_FLAG_IS_AUTH_FW: DWORD = 0x00010000;
pub const FWP_CONDITION_FLAG_IS_RECLASSIFY: DWORD = 0x00020000;
pub const FWP_CONDITION_FLAG_IS_OUTBOUND_PASS_THRU: DWORD = 0x00040000;
pub const FWP_CONDITION_FLAG_IS_INBOUND_PASS_THRU: DWORD = 0x00080000;
pub const FWP_CONDITION_FLAG_IS_CONNECTION_REDIRECTED: DWORD = 0x00100000;
pub const FWP_CONDITION_FLAG_IS_PROXY_CONNECTION: DWORD = 0x00200000;
pub const FWP_CONDITION_FLAG_IS_APPCONTAINER_LOOPBACK: DWORD = 0x00400000;
pub const FWP_CONDITION_FLAG_IS_NON_APPCONTAINER_LOOPBACK: DWORD = 0x00800000;
pub const FWP_CONDITION_FLAG_IS_RESERVED: DWORD = 0x01000000;
pub const FWP_CONDITION_FLAG_IS_HONORING_POLICY_AUTHORIZE: DWORD = 0x02000000;
pub const FWP_CONDITION_REAUTHORIZE_REASON_POLICY_CHANGE: DWORD = 0x00000001;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_ARRIVAL_INTERFACE: DWORD = 0x00000002;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_NEXTHOP_INTERFACE: DWORD = 0x00000004;
pub const FWP_CONDITION_REAUTHORIZE_REASON_PROFILE_CROSSING: DWORD = 0x00000008;
pub const FWP_CONDITION_REAUTHORIZE_REASON_CLASSIFY_COMPLETION: DWORD = 0x00000010;
pub const FWP_CONDITION_REAUTHORIZE_REASON_IPSEC_PROPERTIES_CHANGED: DWORD = 0x00000020;
pub const FWP_CONDITION_REAUTHORIZE_REASON_MID_STREAM_INSPECTION: DWORD = 0x00000040;
pub const FWP_CONDITION_REAUTHORIZE_REASON_SOCKET_PROPERTY_CHANGED: DWORD = 0x00000080;
pub const FWP_CONDITION_REAUTHORIZE_REASON_NEW_INBOUND_MCAST_BCAST_PACKET: DWORD = 0x00000100;
pub const FWP_CONDITION_REAUTHORIZE_REASON_EDP_POLICY_CHANGED: DWORD = 0x00000200;
pub const FWP_CONDITION_REAUTHORIZE_REASON_LOCAL_ADDRESS_UNI_FILTERS_CHANGED: DWORD = 0x00000400;
pub const FWP_CONDITION_REAUTHORIZE_REASON_REMOTE_ADDRESS_UNI_FILTERS_CHANGED: DWORD = 0x00000800;
pub const FWP_CONDITION_REAUTHORIZE_REASON_LOCAL_PORT_UNI_FILTERS_CHANGED: DWORD = 0x00001000;
pub const FWP_CONDITION_REAUTHORIZE_REASON_REMOTE_PORT_UNI_FILTERS_CHANGED: DWORD = 0x00002000;
pub const FWP_CONDITION_REAUTHORIZE_REASON_PROXY_HANDLE_CHANGED: DWORD = 0x00004000;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_IS_SYSTEM_PORT_RPC: DWORD = 0x00000001;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_ALLOW_EDGE_TRAFFIC: DWORD = 0x00000002;
pub const FWP_CONDITION_SOCKET_PROPERTY_FLAG_DENY_EDGE_TRAFFIC: DWORD = 0x00000004;
pub const FWP_CONDITION_L2_IS_NATIVE_ETHERNET: DWORD = 0x00000001;
pub const FWP_CONDITION_L2_IS_WIFI: DWORD = 0x00000002;
pub const FWP_CONDITION_L2_IS_MOBILE_BROADBAND: DWORD = 0x00000004;
pub const FWP_CONDITION_L2_IS_WIFI_DIRECT_DATA: DWORD = 0x00000008;
pub const FWP_CONDITION_L2_IS_VM2VM: DWORD = 0x00000010;
pub const FWP_CONDITION_L2_IS_MALFORMED_PACKET: DWORD = 0x00000020;
pub const FWP_CONDITION_L2_IS_IP_FRAGMENT_GROUP: DWORD = 0x00000040;
pub const FWP_CONDITION_L2_IF_CONNECTOR_PRESENT: DWORD = 0x00000080;

ENUM!{enum FWP_FILTER_ENUM_TYPE {
    FWP_FILTER_ENUM_FULLY_CONTAINED = 0,
    FWP_FILTER_ENUM_OVERLAPPING = FWP_FILTER_ENUM_FULLY_CONTAINED + 1,
    FWP_FILTER_ENUM_TYPE_MAX = FWP_FILTER_ENUM_OVERLAPPING + 1,
}}

pub const FWP_FILTER_ENUM_FLAG_BEST_TERMINATING_MATCH: DWORD = 0x00000001;
pub const FWP_FILTER_ENUM_FLAG_SORTED: DWORD = 0x00000002;
pub const FWP_FILTER_ENUM_FLAG_BOOTTIME_ONLY: DWORD = 0x00000004;
pub const FWP_FILTER_ENUM_FLAG_INCLUDE_BOOTTIME: DWORD = 0x00000008;
pub const FWP_FILTER_ENUM_FLAG_INCLUDE_DISABLED: DWORD = 0x00000010;
pub const FWP_FILTER_ENUM_VALID_FLAGS: DWORD =
    FWP_FILTER_ENUM_FLAG_BEST_TERMINATING_MATCH | FWP_FILTER_ENUM_FLAG_SORTED;
pub const FWP_FILTER_ENUM_FLAG_RESERVED1: DWORD = 0x00000020;
pub const FWP_CALLOUT_FLAG_CONDITIONAL_ON_FLOW: DWORD = 0x00000001;
pub const FWP_CALLOUT_FLAG_ALLOW_OFFLOAD: DWORD = 0x00000002;
pub const FWP_CALLOUT_FLAG_ENABLE_COMMIT_ADD_NOTIFY: DWORD = 0x00000004;
pub const FWP_CALLOUT_FLAG_ALLOW_MID_STREAM_INSPECTION: DWORD = 0x00000008;
pub const FWP_CALLOUT_FLAG_ALLOW_RECLASSIFY: DWORD = 0x00000010;
pub const FWP_CALLOUT_FLAG_RESERVED1: DWORD = 0x00000020;
pub const FWP_CALLOUT_FLAG_ALLOW_RSC: DWORD = 0x00000040;
pub const FWP_CALLOUT_FLAG_ALLOW_L2_BATCH_CLASSIFY: DWORD = 0x00000080;

STRUCT!{struct FWPM_DISPLAY_DATA0 {
    name: *mut wchar_t,
    description: *mut wchar_t,
}}

STRUCT!{struct IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    virtualIfTunnelId: UINT64,
    trafficSelectorId: UINT64,
}}
