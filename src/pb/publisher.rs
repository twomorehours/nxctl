#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProperty {
    #[prost(bytes = "bytes", tag = "1")]
    pub k: ::prost::bytes::Bytes,
    #[prost(bytes = "bytes", tag = "2")]
    pub v: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishRequest {
    #[prost(bool, tag = "1")]
    pub dup: bool,
    #[prost(int32, tag = "2")]
    pub qos: i32,
    #[prost(bool, tag = "3")]
    pub retained: bool,
    #[prost(string, tag = "4")]
    pub topic: ::prost::alloc::string::String,
    #[prost(bytes = "bytes", tag = "5")]
    pub playload: ::prost::bytes::Bytes,
    #[prost(string, tag = "6")]
    pub content_type: ::prost::alloc::string::String,
    #[prost(bytes = "bytes", tag = "7")]
    pub corrlation_data: ::prost::bytes::Bytes,
    #[prost(int32, tag = "8")]
    pub message_expiry: i32,
    #[prost(int32, tag = "9")]
    pub payload_format: i32,
    #[prost(string, tag = "10")]
    pub response_topic: ::prost::alloc::string::String,
    #[prost(uint32, repeated, tag = "11")]
    pub subscription_identifier: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "12")]
    pub user_properties: ::prost::alloc::vec::Vec<UserProperty>,
    #[prost(string, tag = "13")]
    pub from_client_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "14")]
    pub create_at: i64,
    #[prost(string, tag = "15")]
    pub id: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod publish_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PublishServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PublishServiceClient<tonic::transport::Channel> {
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
    impl<T> PublishServiceClient<T>
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
        ) -> PublishServiceClient<InterceptedService<T, F>>
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
            PublishServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn publish(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/publisher.PublishService/Publish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn publish_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::PublishRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/publisher.PublishService/PublishStream");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod publish_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PublishServiceServer."]
    #[async_trait]
    pub trait PublishService: Send + Sync + 'static {
        async fn publish(
            &self,
            request: tonic::Request<super::PublishRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn publish_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::PublishRequest>>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PublishServiceServer<T: PublishService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PublishService> PublishServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PublishServiceServer<T>
    where
        T: PublishService,
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
                "/publisher.PublishService/Publish" => {
                    #[allow(non_camel_case_types)]
                    struct PublishSvc<T: PublishService>(pub Arc<T>);
                    impl<T: PublishService> tonic::server::UnaryService<super::PublishRequest> for PublishSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PublishRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).publish(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PublishSvc(inner);
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
                "/publisher.PublishService/PublishStream" => {
                    #[allow(non_camel_case_types)]
                    struct PublishStreamSvc<T: PublishService>(pub Arc<T>);
                    impl<T: PublishService>
                        tonic::server::ClientStreamingService<super::PublishRequest>
                        for PublishStreamSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::PublishRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).publish_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PublishStreamSvc(inner);
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
    impl<T: PublishService> Clone for PublishServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PublishService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PublishService> tonic::transport::NamedService for PublishServiceServer<T> {
        const NAME: &'static str = "publisher.PublishService";
    }
}
