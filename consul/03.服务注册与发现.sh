 # 服务注册
 curl -X PUT 'http://127.0.0.1:8500/v1/agent/service/register' -H 'content-type:application/json' -d '{"Name":"axum.rs","ID":"axum.rs","Address":"127.0.0.1","Port":54321}'

 # 服务列表
 curl -X GET 'http://127.0.0.1:8500/v1/agent/services' -H 'content-type:application/json'

 # 服务过滤
 curl -X GET 'http://127.0.0.1:8500/v1/agent/services?filter=Service%20%3D%3D%20axum.rs' -H 'content-type:application/json'

 # 取消已注册服务
curl -X PUT 'http://127.0.0.1:8500/v1/agent/service/deregister/axum.rs' -H 'content-type:application/json'