use axum::{response::Html, Form};
use axum_grpc_consul::pb;
use serde::Deserialize;
#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:49527";
    let app = axum::Router::new().route("/calc", axum::routing::get(calc_ui).post(calc));
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn calc_ui() -> Html<&'static str> {
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
    <title>计算器</title>
  </head>
  <body>
    <div class="container is-6 mx-auto my-3">
      <form action="/calc" method="post">
        <h1 class="title">计算器</h1>
        <h3 class="subtitle is-6">
          我们的计算器提供2个整数的加、减、乘、除运算。
        </h3>
        <div class="field">
          <label class="label" for="x">第1个整数</label>
          <div class="control">
            <input
              class="input"
              type="number"
              name="x"
              id="x"
              placeholder="请输入第1个整数"
            />
          </div>
        </div>
        <div class="field">
          <label class="label" for="op">运算符</label>
          <div class="select">
            <select  name="op" id="op">
              <option value="+">加</option>
              <option value="-">减</option>
              <option value="*">乘</option>
              <option value="/">除</option>
            </select>
          </div>
        </div>
        <div class="field">
          <label class="label" for="y">第2个整数</label>
          <div class="control">
            <input
              class="input"
              type="number"
              name="y"
              id="y"
              placeholder="请输入第2个整数"
            />
          </div>
        </div>
        <div class="field">
          <div class="control">
            <button class="button is-link">计算结果</button>
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
struct CalcForm {
    pub x: i32,
    pub y: i32,
    pub op: String,
}

async fn calc(Form(CalcForm { x, y, op }): Form<CalcForm>) -> Result<Html<String>, String> {
    let calc_srv_addr = "http://127.0.0.1:39527";
    let mut client = pb::calculate_service_client::CalculateServiceClient::connect(calc_srv_addr)
        .await
        .map_err(|err| err.to_string())?;
    let req = tonic::Request::new(pb::CalculateRequest { x, y });
    let pb::CalculateRespone { result } = match op.as_str() {
        "+" => client.addition(req).await,
        "-" => client.division(req).await,
        "*" => client.multiplication(req).await,
        "/" => client.division(req).await,
        _ => return Err("不支持的运算".to_string()),
    }
    .map_err(|err| err.to_string())?
    .into_inner();

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
    <title>计算器</title>
  </head>
  <body>
    <div class="container is-6 mx-auto my-3">
      <form action="/calc" method="post">
        <h1 class="title">计算器</h1>
        <h3 class="subtitle is-6">
          我们的计算器提供2个整数的加、减、乘、除运算。
        </h3>
        <div class="field">
          <label class="label" for="x">第1个整数</label>
          <div class="control">
            <input
              class="input"
              type="number"
              name="x"
              id="x"
              placeholder="请输入第1个整数"
            />
          </div>
        </div>
        <div class="field">
          <label class="label" for="op">运算符</label>
          <div class="select">
            <select name="op" id="op">
              <option value="+">加</option>
              <option value="-">减</option>
              <option value="*">乘</option>
              <option value="/">除</option>
            </select>
          </div>
        </div>
        <div class="field">
          <label class="label" for="y">第2个整数</label>
          <div class="control">
            <input
              class="input"
              type="number"
              name="y"
              id="y"
              placeholder="请输入第2个整数"
            />
          </div>
        </div>
        <div class="field">
          <div class="control">
            <button class="button is-link">计算结果</button>
          </div>
        </div>
      </form>

      <article class="message is-info mt-3">
        <div class="message-body">{x} {op} {y} = {result}</div>
      </article>
    </div>
  </body>
</html>
"#,
        result = result,
        x = x,
        y = y,
        op = &op
    );
    Ok(Html(tpl))
}
