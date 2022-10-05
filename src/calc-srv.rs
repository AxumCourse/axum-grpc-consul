use axum_grpc_consul::pb;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:39527";
    let calc = CalcService::default();
    println!("CalcServer 监听于 {}", addr);
    tonic::transport::Server::builder()
        .add_service(pb::calculate_service_server::CalculateServiceServer::new(
            calc,
        ))
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}

#[derive(Default)]
pub struct CalcService {}

#[tonic::async_trait]
impl pb::calculate_service_server::CalculateService for CalcService {
    async fn addition(
        &self,
        request: tonic::Request<pb::CalculateRequest>,
    ) -> Result<tonic::Response<pb::CalculateRespone>, tonic::Status> {
        let pb::CalculateRequest { x, y } = request.into_inner();
        Ok(tonic::Response::new(pb::CalculateRespone { result: x + y }))
    }
    async fn subtraction(
        &self,
        request: tonic::Request<pb::CalculateRequest>,
    ) -> Result<tonic::Response<pb::CalculateRespone>, tonic::Status> {
        let pb::CalculateRequest { x, y } = request.into_inner();
        Ok(tonic::Response::new(pb::CalculateRespone { result: x - y }))
    }
    async fn multiplication(
        &self,
        request: tonic::Request<pb::CalculateRequest>,
    ) -> Result<tonic::Response<pb::CalculateRespone>, tonic::Status> {
        let pb::CalculateRequest { x, y } = request.into_inner();
        Ok(tonic::Response::new(pb::CalculateRespone { result: x * y }))
    }
    async fn division(
        &self,
        request: tonic::Request<pb::CalculateRequest>,
    ) -> Result<tonic::Response<pb::CalculateRespone>, tonic::Status> {
        let pb::CalculateRequest { x, y } = request.into_inner();
        if y == 0 {
            return Err(tonic::Status::new(
                tonic::Code::InvalidArgument,
                "除数不能为0".to_string(),
            ));
        }
        Ok(tonic::Response::new(pb::CalculateRespone { result: x / y }))
    }
}
