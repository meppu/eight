use async_trait::async_trait;

tonic::include_proto!("eight");

pub struct MyEightRpc;

#[async_trait]
impl eight_server::Eight for MyEightRpc {
    async fn rpc(
        &self,
        request: tonic::Request<EightRequest>,
    ) -> Result<tonic::Response<EightReply>, tonic::Status> {
        Ok(tonic::Response::new(EightReply {
            message: format!("Echoing back: {}", request.get_ref().message),
        }))
    }
}
