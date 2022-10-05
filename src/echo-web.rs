use axum::{response::Html, Form};
use axum_grpc_consul::{consul_api, pb};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:29527";
    tokio::spawn(register(addr));

    let app = axum::Router::new().route("/echo", axum::routing::get(echo_ui).post(echo));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
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
    let reg = consul_api::Registration::simple("echo-web", addr, port);
    cs.register(&reg).await.unwrap();
    println!("服务注册成功");
}
async fn discovery(service_id: &str) -> Result<String, String> {
    println!("正在发现服务 {}", service_id);
    let opt = consul_api::ConsulOption::default();
    let cs = consul_api::Consul::new(opt).unwrap();
    let filter = consul_api::Filter::ID(service_id.into());
    let srv = cs
        .get_service(&filter)
        .await
        .map_err(|err| err.to_string())?;
    if let Some(srv) = srv {
        return Ok(format!("http://{}:{}", srv.address, srv.port));
    }
    Err("没有发现指定的服务".to_string())
}

async fn echo_ui() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="zh-Hans">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link
      rel="stylesheet"
      href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css"
    />
    <title>ECHO</title>
  </head>
  <body>
    <div class="container is-6 mx-auto my-3">
      <form action="/echo" method="post">
        <h1 class="title">ECHO</h1>
        <h3 class="subtitle is-6">
          将信息发送给服务器，服务器将把信息回显给你。
        </h3>
        <div class="field">
          <label class="label" for="message">信息</label>
          <div class="control">
            <input
              class="input"
              type="text"
              name="message"
              id="message"
              placeholder="输入你要发送的信息"
            />
          </div>
        </div>
        <div class="field">
          <div class="control">
            <button class="button is-primary">发送</button>
          </div>
        </div>
      </form>
    </div>
  </body>
</html>
"#,
    )
}

#[derive(Deserialize)]
struct EchoInput {
    pub message: String,
}
async fn echo(Form(EchoInput { message }): Form<EchoInput>) -> Result<Html<String>, String> {
    let echo_srv_addr = discovery("echo-srv").await?;
    let mut client = pb::echo_serivce_client::EchoSerivceClient::connect(echo_srv_addr)
        .await
        .map_err(|err| err.to_string())?;
    let req = tonic::Request::new(pb::EchoRequest { message });
    let pb::EchoResponse { message, prefix } = client
        .echo(req)
        .await
        .map_err(|err| err.to_string())?
        .into_inner();
    let prefix = prefix.unwrap_or("".to_string());
    let tpl = format!(
        r#"<!DOCTYPE html>
<html lang="zh-Hans">
  <head>
    <meta charset="UTF-8" />
    <meta http-equiv="X-UA-Compatible" content="IE=edge" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <link
      rel="stylesheet"
      href="https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css"
    />
    <title>ECHO</title>
  </head>
  <body>
    <div class="container is-6 mx-auto my-3">
      <form action="/echo" method="post">
        <h1 class="title">ECHO</h1>
        <h3 class="subtitle is-6">
          将信息发送给服务器，服务器将把信息回显给你。
        </h3>
        <div class="field">
          <label class="label" for="message">信息</label>
          <div class="control">
            <input
              class="input"
              type="text"
              name="message"
              id="message"
              placeholder="输入你要发送的信息"
            />
          </div>
        </div>
        <div class="field">
          <div class="control">
            <button class="button is-primary">发送</button>
          </div>
        </div>
      </form>

      <article class="message is-info mt-3">
        <div class="message-body">{} {}</div>
      </article>
    </div>
  </body>
</html>
"#,
        prefix, message
    );
    Ok(Html(tpl))
}
