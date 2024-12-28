// This file is @generated by prost-build.
#[derive(sea_orm::FromJsonQueryResult)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Privilege {
    #[prost(string, tag = "1")]
    pub grantor: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    #[prost(enumeration = "PrivilegeType", optional, tag = "3")]
    pub privilege_type: ::core::option::Option<i32>,
    #[prost(bool, tag = "4")]
    pub is_grantable: bool,
}
#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrivilegeType {
    All = 0,
    Select = 1,
    Insert = 2,
    Update = 3,
    Delete = 4,
    Truncate = 5,
    References = 6,
    Trigger = 7,
}
impl PrivilegeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PrivilegeType::All => "ALL",
            PrivilegeType::Select => "SELECT",
            PrivilegeType::Insert => "INSERT",
            PrivilegeType::Update => "UPDATE",
            PrivilegeType::Delete => "DELETE",
            PrivilegeType::Truncate => "TRUNCATE",
            PrivilegeType::References => "REFERENCES",
            PrivilegeType::Trigger => "TRIGGER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALL" => Some(Self::All),
            "SELECT" => Some(Self::Select),
            "INSERT" => Some(Self::Insert),
            "UPDATE" => Some(Self::Update),
            "DELETE" => Some(Self::Delete),
            "TRUNCATE" => Some(Self::Truncate),
            "REFERENCES" => Some(Self::References),
            "TRIGGER" => Some(Self::Trigger),
            _ => None,
        }
    }
}
