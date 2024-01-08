# in bashrc
# if [ $(pwd | grep -c "$REPOS/rust-cp") -gt 0 ]; then
#     source $REPOS/rust-cp/cmd.sh
# fi

contest_id=$(basename $(pwd))

build() {
  touch src/bin/$1.rs
  echo -e "cargo build --bin $contest_id-$1 --release"
  cargo build --bin $contest_id-$1 --release
}

build-debug() {
  touch src/bin/$1.rs
  echo -e "cargo build --bin $contest_id-$1"
  cargo build --bin $contest_id-$1
}

run() {
  build-debug $1
  RUST_BACKTRACE=1
  cargo run --bin $contest_id-$1
}

ts() {
  build-debug $1
  if [ $? -ne 0 ]; then
    echo "build failed"
    return
  fi
  cargo compete test $1 --debug
  if [ $? -ne 0 ]; then
    echo "-- wa"
  else
    echo "-- ac"
  fi
}

sub() {
  build $1
  next_problem=$(echo $1 | tr 'a-y' 'b-z')
  if [ -e src/bin/$next_problem.rs ]; then
    nohup code src/bin/$next_problem.rs
  fi
  cargo compete submit $1 --release
  if [ $? -ne 0 ]; then
    echo "-- wa"
    code src/bin/$1.rs
    return
  else
    echo "-- ac"
  fi
}

o() {
  code src/bin/$1.rs
}
