# MacOS - Homebrew
brew tap hashicorp/tap
brew install hashicorp/tap/consul

# MacOS - Binary
wget https://releases.hashicorp.com/consul/1.13.2/consul_1.13.2_darwin_amd64.zip
unzip consul_1.13.2_darwin_amd64.zip

# Linux - apt
wget -O- https://apt.releases.hashicorp.com/gpg | gpg --dearmor | sudo tee /usr/share/keyrings/hashicorp-archive-keyring.gpg
echo "deb [signed-by=/usr/share/keyrings/hashicorp-archive-keyring.gpg] https://apt.releases.hashicorp.com $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/hashicorp.list
sudo apt update && sudo apt install consul

# Linux - Binray
wget https://releases.hashicorp.com/consul/1.13.2/consul_1.13.2_linux_amd64.zip
unzip consul_1.13.2_linux_amd64.zip

# Windows
# 下载：https://releases.hashicorp.com/consul/1.13.2/consul_1.13.2_windows_amd64.zip
# 解压，然后添加到系统环境变量
