apt update && apt upgrade -y

apt install curl -y
#install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh < echo 1
. "$HOME/.cargo/env"


#install play for audio
apt install sox -y && apt install pulseaudio -y

"2"
"37"




