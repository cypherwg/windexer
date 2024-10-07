use crate::grpc::service::WindexerService;
use tonic::transport::Server;
use windexer_proto::windexer_server::WindexerServer;

pub struct GrpcServer;

impl GrpcServer {
    pub async fn run(
        addr: String,
        service: WindexerService,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let addr = addr.parse()?;
        let server = WindexerServer::new(service);

        Server::builder().add_service(server).serve(addr).await?;

        Ok(())
    }
}
