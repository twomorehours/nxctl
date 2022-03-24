// for rpc server

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topicfilter {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub qos: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubRequest {
    #[prost(string, tag = "1")]
    pub clientid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub topicfilters: ::prost::alloc::vec::Vec<Topicfilter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubRequest {
    #[prost(string, tag = "1")]
    pub clientid: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub topicfilters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubResponse {
    #[prost(int32, repeated, tag = "1")]
    pub codes: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnsubResponse {
    #[prost(int32, repeated, tag = "2")]
    pub codes: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubRequest {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// the field will be ignored when used as UpdateTopic's parameter
    #[prost(int32, tag = "2")]
    pub qos: i32,
    #[prost(bytes = "bytes", tag = "3")]
    pub retain: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetainRequest {
    #[prost(string, repeated, tag = "1")]
    pub topicfilters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetainInfo {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    #[prost(bytes = "bytes", tag = "2")]
    pub retain: ::prost::bytes::Bytes,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetainResponse {
    #[prost(message, repeated, tag = "1")]
    pub retains: ::prost::alloc::vec::Vec<RetainInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetsubRequest {
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetsubResponse {
    #[prost(string, repeated, tag = "1")]
    pub subs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetsubStreamResponse {
    /// err = 0 indicates no error occurs
    #[prost(int32, tag = "1")]
    pub err: i32,
    #[prost(string, repeated, tag = "2")]
    pub subs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetsubStreamResponses {
    #[prost(message, repeated, tag = "1")]
    pub subs: ::prost::alloc::vec::Vec<GetsubStreamResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListsubRequest {
    /// could be empty
    #[prost(string, tag = "1")]
    pub clientid: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub topicfilters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// if qos = -1, it will not be used to filter the responses
    #[prost(int32, tag = "3")]
    pub qos: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    #[prost(string, tag = "1")]
    pub clientid: ::prost::alloc::string::String,
    /// for share sub, it's $share/group_name/xxx/xxx
    #[prost(string, tag = "2")]
    pub topic: ::prost::alloc::string::String,
    /// for normal sub, it's topic name
    #[prost(int32, tag = "3")]
    pub qos: i32,
    /// for share sub, it's a specific topic name
    #[prost(string, tag = "4")]
    pub match_topic: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListsubResponse {
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<Subscription>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    #[prost(string, tag = "1")]
    pub topicfilter: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    #[prost(string, repeated, tag = "1")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod pub_sub_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PubSubServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PubSubServiceClient<tonic::transport::Channel> {
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
    impl<T> PubSubServiceClient<T>
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
        ) -> PubSubServiceClient<InterceptedService<T, F>>
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
            PubSubServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn sub(
            &mut self,
            request: impl tonic::IntoRequest<super::SubRequest>,
        ) -> Result<tonic::Response<super::SubResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/Sub");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn unsub(
            &mut self,
            request: impl tonic::IntoRequest<super::UnsubRequest>,
        ) -> Result<tonic::Response<super::UnsubResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/Unsub");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::PubRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/CreateTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn del_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::PubRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/DelTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::PubRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/UpdateTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_retains(
            &mut self,
            request: impl tonic::IntoRequest<super::RetainRequest>,
        ) -> Result<tonic::Response<super::RetainResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/GetRetains");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_subs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetsubRequest>,
        ) -> Result<tonic::Response<super::GetsubResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/GetSubs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_subs_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::GetsubRequest>,
        ) -> Result<tonic::Response<super::GetsubStreamResponses>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/GetSubsStream");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn list_subs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListsubRequest>,
        ) -> Result<tonic::Response<super::ListsubResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/ListSubs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn search_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> Result<tonic::Response<super::SearchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/pubsub.PubSubService/SearchTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod pub_sub_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PubSubServiceServer."]
    #[async_trait]
    pub trait PubSubService: Send + Sync + 'static {
        async fn sub(
            &self,
            request: tonic::Request<super::SubRequest>,
        ) -> Result<tonic::Response<super::SubResponse>, tonic::Status>;
        async fn unsub(
            &self,
            request: tonic::Request<super::UnsubRequest>,
        ) -> Result<tonic::Response<super::UnsubResponse>, tonic::Status>;
        async fn create_topic(
            &self,
            request: tonic::Request<super::PubRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn del_topic(
            &self,
            request: tonic::Request<super::PubRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn update_topic(
            &self,
            request: tonic::Request<super::PubRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        async fn get_retains(
            &self,
            request: tonic::Request<super::RetainRequest>,
        ) -> Result<tonic::Response<super::RetainResponse>, tonic::Status>;
        async fn get_subs(
            &self,
            request: tonic::Request<super::GetsubRequest>,
        ) -> Result<tonic::Response<super::GetsubResponse>, tonic::Status>;
        async fn get_subs_stream(
            &self,
            request: tonic::Request<tonic::Streaming<super::GetsubRequest>>,
        ) -> Result<tonic::Response<super::GetsubStreamResponses>, tonic::Status>;
        async fn list_subs(
            &self,
            request: tonic::Request<super::ListsubRequest>,
        ) -> Result<tonic::Response<super::ListsubResponse>, tonic::Status>;
        async fn search_topic(
            &self,
            request: tonic::Request<super::SearchRequest>,
        ) -> Result<tonic::Response<super::SearchResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PubSubServiceServer<T: PubSubService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PubSubService> PubSubServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PubSubServiceServer<T>
    where
        T: PubSubService,
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
                "/pubsub.PubSubService/Sub" => {
                    #[allow(non_camel_case_types)]
                    struct SubSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::SubRequest> for SubSvc<T> {
                        type Response = super::SubResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sub(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubSvc(inner);
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
                "/pubsub.PubSubService/Unsub" => {
                    #[allow(non_camel_case_types)]
                    struct UnsubSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::UnsubRequest> for UnsubSvc<T> {
                        type Response = super::UnsubResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnsubRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).unsub(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnsubSvc(inner);
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
                "/pubsub.PubSubService/CreateTopic" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTopicSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::PubRequest> for CreateTopicSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PubRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateTopicSvc(inner);
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
                "/pubsub.PubSubService/DelTopic" => {
                    #[allow(non_camel_case_types)]
                    struct DelTopicSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::PubRequest> for DelTopicSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PubRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).del_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DelTopicSvc(inner);
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
                "/pubsub.PubSubService/UpdateTopic" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTopicSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::PubRequest> for UpdateTopicSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PubRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateTopicSvc(inner);
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
                "/pubsub.PubSubService/GetRetains" => {
                    #[allow(non_camel_case_types)]
                    struct GetRetainsSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::RetainRequest> for GetRetainsSvc<T> {
                        type Response = super::RetainResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetainRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_retains(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRetainsSvc(inner);
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
                "/pubsub.PubSubService/GetSubs" => {
                    #[allow(non_camel_case_types)]
                    struct GetSubsSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::GetsubRequest> for GetSubsSvc<T> {
                        type Response = super::GetsubResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetsubRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_subs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSubsSvc(inner);
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
                "/pubsub.PubSubService/GetSubsStream" => {
                    #[allow(non_camel_case_types)]
                    struct GetSubsStreamSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService>
                        tonic::server::ClientStreamingService<super::GetsubRequest>
                        for GetSubsStreamSvc<T>
                    {
                        type Response = super::GetsubStreamResponses;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::GetsubRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_subs_stream(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSubsStreamSvc(inner);
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
                "/pubsub.PubSubService/ListSubs" => {
                    #[allow(non_camel_case_types)]
                    struct ListSubsSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::ListsubRequest> for ListSubsSvc<T> {
                        type Response = super::ListsubResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListsubRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_subs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSubsSvc(inner);
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
                "/pubsub.PubSubService/SearchTopic" => {
                    #[allow(non_camel_case_types)]
                    struct SearchTopicSvc<T: PubSubService>(pub Arc<T>);
                    impl<T: PubSubService> tonic::server::UnaryService<super::SearchRequest> for SearchTopicSvc<T> {
                        type Response = super::SearchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchTopicSvc(inner);
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
    impl<T: PubSubService> Clone for PubSubServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PubSubService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PubSubService> tonic::transport::NamedService for PubSubServiceServer<T> {
        const NAME: &'static str = "pubsub.PubSubService";
    }
}
