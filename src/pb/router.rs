#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutRequest {
    #[prost(string, tag = "1")]
    pub clientid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub acceptor_info: ::core::option::Option<AcceptorInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceptorInfo {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetRequest {
    #[prost(string, repeated, tag = "1")]
    pub clientid_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub clientid: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub acceptor_info: ::core::option::Option<AcceptorInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetResponse {
    #[prost(map = "string, message", tag = "1")]
    pub client_acceptor_map:
        ::std::collections::HashMap<::prost::alloc::string::String, AcceptorInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamGetResponse {
    #[prost(map = "string, string", tag = "1")]
    pub client_map:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelRequest {
    #[prost(string, tag = "1")]
    pub clientid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub addr: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod route_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct RouteServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RouteServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RouteServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RouteServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            RouteServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn put(
            &mut self,
            request: impl tonic::IntoRequest<super::PutRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/router.RouteService/Put");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_get(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetRequest>,
        ) -> Result<tonic::Response<super::BatchGetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/router.RouteService/BatchGet");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::GetRequest>,
        ) -> Result<tonic::Response<super::StreamGetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/router.RouteService/GetStream");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/router.RouteService/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn del(
            &mut self,
            request: impl tonic::IntoRequest<super::DelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/router.RouteService/Del");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod route_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RouteServiceServer."]
    #[async_trait]
    pub trait RouteService: Send + Sync + 'static {
        async fn put(
            &self,
            request: tonic::Request<super::PutRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn batch_get(
            &self,
            request: tonic::Request<super::BatchGetRequest>,
        ) -> Result<tonic::Response<super::BatchGetResponse>, tonic::Status>;
        async fn get_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::GetRequest>>,
        ) -> Result<tonic::Response<super::StreamGetResponse>, tonic::Status>;
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        async fn del(
            &self,
            request: tonic::Request<super::DelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RouteServiceServer<T: RouteService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RouteService> RouteServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RouteServiceServer<T>
    where
        T: RouteService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/router.RouteService/Put" => {
                    #[allow(non_camel_case_types)]
                    struct PutSvc<T: RouteService>(pub Arc<T>);
                    impl<T: RouteService> tonic::server::UnaryService<super::PutRequest> for PutSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).put(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/router.RouteService/BatchGet" => {
                    #[allow(non_camel_case_types)]
                    struct BatchGetSvc<T: RouteService>(pub Arc<T>);
                    impl<T: RouteService> tonic::server::UnaryService<super::BatchGetRequest> for BatchGetSvc<T> {
                        type Response = super::BatchGetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchGetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).batch_get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchGetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/router.RouteService/GetStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetStreamSvc<T: RouteService>(pub Arc<T>);
                    impl<T: RouteService> tonic::server::ClientStreamingService<super::GetRequest> for GetStreamSvc<T> {
                        type Response = super::StreamGetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::GetRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStreamSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/router.RouteService/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: RouteService>(pub Arc<T>);
                    impl<T: RouteService> tonic::server::UnaryService<super::GetRequest> for GetSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/router.RouteService/Del" => {
                    #[allow(non_camel_case_types)]
                    struct DelSvc<T: RouteService>(pub Arc<T>);
                    impl<T: RouteService> tonic::server::UnaryService<super::DelRequest> for DelSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).del(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: RouteService> Clone for RouteServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RouteService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RouteService> tonic::transport::NamedService for RouteServiceServer<T> {
        const NAME: &'static str = "router.RouteService";
    }
}
