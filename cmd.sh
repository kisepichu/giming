# in bashrc
# if [ $(pwd | grep -c "$REPOS/rust-cp") -gt 0 ]; then
#     source $REPOS/rust-cp/cmd.sh
# fi

contest_id=$(basename $(pwd))

build() {
    echo -e "cargo build --bin $contest_id-$1"
    cargo build --bin $contest_id-$1
}

run() {
    build $1
    RUST_BACKTRACE=1
    cargo run --bin $contest_id-$1
}

ts() {
    build $1
    if [ $? -ne 0 ]; then
        echo "build failed"
        return
    fi
    cargo compete test $1 --debug
}

sub() {
    build $1
    next_problem=$(echo $1 | tr 'a-y' 'b-z')
    nohup code src/bin/$next_problem.rs
    cargo compete submit $1
}

o() {
    code src/bin/$1.rs
}
