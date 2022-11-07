if ! command -v cargo &> /dev/null
then
    echo "cargo could not be found, installing rust on the machine"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

cargo install orion-ssg
orion-ssg build
