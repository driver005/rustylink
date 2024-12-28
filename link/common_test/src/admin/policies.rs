// This file is @generated by prost-build.
#[derive(sea_orm::FromJsonQueryResult)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPoliciesRequest {
    #[prost(bool, optional, tag = "1")]
    pub include_system_schemas: ::core::option::Option<bool>,
    #[prost(string, repeated, tag = "2")]
    pub included_schemas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub excluded_schemas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "4")]
    pub limit: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub offset: ::core::option::Option<i32>,
}
#[derive(sea_orm::FromJsonQueryResult)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicieRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(sea_orm::FromJsonQueryResult)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicieRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub definition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub check: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(sea_orm::FromJsonQueryResult)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicieRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub schema: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub definition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub check: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub command: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "8")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(sea_orm::FromJsonQueryResult)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicieRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(sea_orm::FromJsonQueryResult)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyResponce {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub schema: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub table_id: i32,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "Action", optional, tag = "6")]
    pub action: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "7")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "Command", optional, tag = "8")]
    pub command: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "9")]
    pub definition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub check: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    Permissive = 0,
    Restrictive = 1,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Permissive => "PERMISSIVE",
            Action::Restrictive => "RESTRICTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERMISSIVE" => Some(Self::Permissive),
            "RESTRICTIVE" => Some(Self::Restrictive),
            _ => None,
        }
    }
}
#[derive(sea_orm::EnumIter, sea_orm::DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Command {
    Select = 0,
    Insert = 1,
    Update = 2,
    Delete = 3,
    All = 4,
}
impl Command {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Command::Select => "SELECT",
            Command::Insert => "INSERT",
            Command::Update => "UPDATE",
            Command::Delete => "DELETE",
            Command::All => "ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SELECT" => Some(Self::Select),
            "INSERT" => Some(Self::Insert),
            "UPDATE" => Some(Self::Update),
            "DELETE" => Some(Self::Delete),
            "ALL" => Some(Self::All),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod policies_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PoliciesServer.
    #[async_trait]
    pub trait Policies: Send + Sync + 'static {
        async fn get_policies(
            &self,
            request: tonic::Request<super::GetPoliciesRequest>,
        ) -> std::result::Result<tonic::Response<super::PolicyResponce>, tonic::Status>;
        async fn get_policie(
            &self,
            request: tonic::Request<super::GetPolicieRequest>,
        ) -> std::result::Result<tonic::Response<super::PolicyResponce>, tonic::Status>;
        async fn update_policie(
            &self,
            request: tonic::Request<super::UpdatePolicieRequest>,
        ) -> std::result::Result<tonic::Response<super::PolicyResponce>, tonic::Status>;
        async fn create_policie(
            &self,
            request: tonic::Request<super::CreatePolicieRequest>,
        ) -> std::result::Result<tonic::Response<super::PolicyResponce>, tonic::Status>;
        async fn delete_policie(
            &self,
            request: tonic::Request<super::DeletePolicieRequest>,
        ) -> std::result::Result<tonic::Response<super::PolicyResponce>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PoliciesServer<T: Policies> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: Policies> PoliciesServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PoliciesServer<T>
    where
        T: Policies,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/policies.Policies/GetPolicies" => {
                    #[allow(non_camel_case_types)]
                    struct GetPoliciesSvc<T: Policies>(pub Arc<T>);
                    impl<
                        T: Policies,
                    > tonic::server::UnaryService<super::GetPoliciesRequest>
                    for GetPoliciesSvc<T> {
                        type Response = super::PolicyResponce;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPoliciesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Policies>::get_policies(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetPoliciesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/policies.Policies/GetPolicie" => {
                    #[allow(non_camel_case_types)]
                    struct GetPolicieSvc<T: Policies>(pub Arc<T>);
                    impl<
                        T: Policies,
                    > tonic::server::UnaryService<super::GetPolicieRequest>
                    for GetPolicieSvc<T> {
                        type Response = super::PolicyResponce;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPolicieRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Policies>::get_policie(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetPolicieSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/policies.Policies/UpdatePolicie" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePolicieSvc<T: Policies>(pub Arc<T>);
                    impl<
                        T: Policies,
                    > tonic::server::UnaryService<super::UpdatePolicieRequest>
                    for UpdatePolicieSvc<T> {
                        type Response = super::PolicyResponce;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePolicieRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Policies>::update_policie(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdatePolicieSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/policies.Policies/CreatePolicie" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePolicieSvc<T: Policies>(pub Arc<T>);
                    impl<
                        T: Policies,
                    > tonic::server::UnaryService<super::CreatePolicieRequest>
                    for CreatePolicieSvc<T> {
                        type Response = super::PolicyResponce;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePolicieRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Policies>::create_policie(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreatePolicieSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/policies.Policies/DeletePolicie" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePolicieSvc<T: Policies>(pub Arc<T>);
                    impl<
                        T: Policies,
                    > tonic::server::UnaryService<super::DeletePolicieRequest>
                    for DeletePolicieSvc<T> {
                        type Response = super::PolicyResponce;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePolicieRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Policies>::delete_policie(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeletePolicieSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Policies> Clone for PoliciesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Policies> tonic::server::NamedService for PoliciesServer<T> {
        const NAME: &'static str = "policies.Policies";
    }
}
