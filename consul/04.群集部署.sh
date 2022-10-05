### server节点1(主节点)
# 节点名称
NODE_NAME="server-01" && \
# 数据保存目录
DATA_DIR="/var/consul/data/${NODE_NAME}" && \
# 集群中节点数量
BOOTSTRAP_EXPECT=3 && \
mkdir -p $DATA_DIR && \
consul agent -server \
	-bind=0.0.0.0 \
	-bootstrap-expect=$BOOTSTRAP_EXPECT \
	-data-dir=$DATA_DIR \
	-node=$NODE_NAME

### server节点2
# 节点名称
NODE_NAME="server-02" && \
# 数据保存目录
DATA_DIR="/var/consul/data/${NODE_NAME}" && \
# 集群中节点数量
BOOTSTRAP_EXPECT=3 && \
mkdir -p $DATA_DIR && \
consul agent -server \
	-bind=0.0.0.0 \
	-bootstrap-expect=$BOOTSTRAP_EXPECT \
	-data-dir=$DATA_DIR \
	-node=$NODE_NAME

### server节点3
# 节点名称
NODE_NAME="server-03" && \
# 数据保存目录
DATA_DIR="/var/consul/data/${NODE_NAME}" && \
# 集群中节点数量
BOOTSTRAP_EXPECT=3 && \
mkdir -p $DATA_DIR && \
consul agent -server \
	-bind=0.0.0.0 \
	-bootstrap-expect=$BOOTSTRAP_EXPECT \
	-data-dir=$DATA_DIR \
	-node=$NODE_NAME

### client 节点
# 节点名称
NODE_NAME="client-01" && \
# 数据保存目录
DATA_DIR="/var/consul/data/${NODE_NAME}" && \
mkdir -p $DATA_DIR && \
consul agent \
	-client=0.0.0.0 \
	-bind=0.0.0.0 \
	-data-dir=$DATA_DIR \
	-node=$NODE_NAME

### 加入主节点
### -- server02 server03 和 client 节点都需要进行这个操作
consul join '主节点IPV4地址'
# 或
consul join '[主节点IPV6地址]'

### 查询节点
consul members