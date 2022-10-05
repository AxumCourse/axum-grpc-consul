DST_DIR=/usr/local/consul/
CONSUL_VERION=1.13.2
TMP_NAME="/tmp/consul.${CONSUL_VERION}.zip"
RC_FILE=~/.bashrc
wget -4 -O $TMP_NAME  "https://releases.hashicorp.com/consul/${CONSUL_VERION}/consul_${CONSUL_VERION}_linux_amd64.zip" && \
unzip $TMP_NAME -d $DST_DIR && \
chmod a+x "${DST_DIR}/consul" && \
echo "export PATH=\$PATH:${DST_DIR}" >> $RC_FILE && \
source $RC_FILE && \
rm -rf $TMP_NAME