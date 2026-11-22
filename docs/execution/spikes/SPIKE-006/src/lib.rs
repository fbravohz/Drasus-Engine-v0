pub mod wire { tonic::include_proto!("drasus.spike.v1"); }

#[cfg(test)]
mod tests {
    use super::wire::{probe_client::ProbeClient, probe_server::{Probe, ProbeServer}, CheckRequest, CheckReply, WatchRequest, WatchReply};
    use std::pin::Pin;
    use tokio_stream::{Stream, wrappers::TcpListenerStream};
    use tonic::{Request, Response, Status, transport::Server};

    #[derive(Default)]
    struct Service;

    #[tonic::async_trait]
    impl Probe for Service {
        async fn check(&self, _: Request<CheckRequest>) -> Result<Response<CheckReply>, Status> {
            Ok(Response::new(CheckReply { ready: true }))
        }
        type WatchStream = Pin<Box<dyn Stream<Item=Result<WatchReply, Status>> + Send>>;
        async fn watch(&self, request: Request<WatchRequest>) -> Result<Response<Self::WatchStream>, Status> {
            let after = request.into_inner().after;
            if after > 2 { return Err(Status::out_of_range("CURSOR_GAP")); }
            let events = (after + 1..=2).map(move |sequence| Ok(WatchReply { sequence, initial: after == 0 && sequence == 1 }));
            Ok(Response::new(Box::pin(tokio_stream::iter(events))))
        }
    }

    fn authenticate(request: Request<()>) -> Result<Request<()>, Status> {
        match request.metadata().get("authorization").and_then(|v| v.to_str().ok()) {
            Some("Bearer session-secret") => Ok(request),
            _ => Err(Status::unauthenticated("AUTH_REQUIRED")),
        }
    }

    #[tokio::test]
    async fn local_auth_unary_and_recoverable_stream() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let (stop_tx, stop_rx) = tokio::sync::oneshot::channel::<()>();
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(ProbeServer::with_interceptor(Service, authenticate))
                .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async { let _ = stop_rx.await; })
                .await.unwrap();
        });
        let endpoint = format!("http://{address}");
        let mut client = ProbeClient::connect(endpoint).await.unwrap();
        assert_eq!(client.check(CheckRequest {}).await.unwrap_err().code(), tonic::Code::Unauthenticated);
        let mut invalid = Request::new(CheckRequest {});
        invalid.metadata_mut().insert("authorization", "Bearer wrong".parse().unwrap());
        let denial = client.check(invalid).await.unwrap_err();
        assert_eq!(denial.code(), tonic::Code::Unauthenticated);
        assert_eq!(denial.message(), "AUTH_REQUIRED");
        assert!(!denial.message().contains("session-secret"));
        let mut valid = Request::new(CheckRequest {});
        valid.metadata_mut().insert("authorization", "Bearer session-secret".parse().unwrap());
        assert!(client.check(valid).await.unwrap().into_inner().ready);
        let mut first = Request::new(WatchRequest { after: 0 });
        first.metadata_mut().insert("authorization", "Bearer session-secret".parse().unwrap());
        let mut stream = client.watch(first).await.unwrap().into_inner();
        let initial = stream.message().await.unwrap().unwrap();
        assert_eq!((initial.sequence, initial.initial), (1, true));
        drop(stream);
        let mut resume = Request::new(WatchRequest { after: 1 });
        resume.metadata_mut().insert("authorization", "Bearer session-secret".parse().unwrap());
        let mut stream = client.watch(resume).await.unwrap().into_inner();
        assert_eq!(stream.message().await.unwrap().unwrap().sequence, 2);
        let mut gap = Request::new(WatchRequest { after: 3 });
        gap.metadata_mut().insert("authorization", "Bearer session-secret".parse().unwrap());
        assert_eq!(client.watch(gap).await.unwrap_err().code(), tonic::Code::OutOfRange);
        let _ = stop_tx.send(());
        server.await.unwrap();
    }
}
