use proto::eight_server::{EightRpc, EightRpcServer};
use proto::{EightReply, EightRequest};

pub struct MyEightRpc;

#[async_trait]
impl eight_server::Eight for MyEightRpc {
    async fn eightrpc(
        &self,
        request: tonic::Request<EightRequest>,
    ) -> Result<tonic::Response<EightReply>, tonic::Status> {
        Ok(tonic::Response::new(EightReply {
            message: format!("Echoing back: {}", request.get_ref().message),
        }))
    }
}
