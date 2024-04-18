apt-get update
apt-get install -y \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl

curl https://sh.rustup.rs -sSf | sh -s -- -y 
rustup install stable
rustup component add rustfmt
rustup component add clippy 
