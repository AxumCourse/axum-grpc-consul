use std::sync::Arc;

use axum_grpc_consul::{consul_api, pb};
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:19527";
    let echo = EchoService {
        prefix: Arc::new("【小猪佩奇】".to_string()),
    };

    tokio::spawn(register(addr));

    println!("EchoServer 监听于 {}", addr);

    tonic::transport::Server::builder()
        .add_service(pb::echo_serivce_server::EchoSerivceServer::new(echo))
        .serve(addr.parse().unwrap())
        .await
        .unwrap();
}

async fn register(addr: &str) {
    println!("正在注册服务");
    let addrs: Vec<&str> = addr.split(":").collect();
    let addr = addrs[0];
    let port: i32 = addrs[1].parse().unwrap();
    let opt = consul_api::ConsulOption::default();
    let cs = consul_api::Consul::new(opt).unwrap();
    let reg = consul_api::Registration::simple("echo-srv", addr, port);
    cs.register(&reg).await.unwrap();
    println!("服务注册成功");
}

pub struct EchoService {
    pub prefix: Arc<String>,
}

#[tonic::async_trait]
impl pb::echo_serivce_server::EchoSerivce for EchoService {
    async fn echo(
        &self,
        request: tonic::Request<pb::EchoRequest>,
    ) -> Result<tonic::Response<pb::EchoResponse>, tonic::Status> {
        let pb::EchoRequest { message } = request.into_inner();
        let resp = pb::EchoResponse {
            message,
            prefix: Some(self.prefix.to_string()),
        };
        Ok(tonic::Response::new(resp))
    }
}
