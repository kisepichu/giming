p=$(pwd)
render_template=render-rust.yaml
contest_id=$1
if [ -z $contest_id ]; then
    echo "Usage: init.sh <contest_id>"
    exit 1
fi

cd ./autotaker
mkdir -p $contest_id
cd $contest_id
echo -e "$ATCODER_USER\n$ATCODER_PASS\n" | kyopro -l -r $p/$render_template $contest_id
cd ../..

cd ./compete
cargo compete new $contest_id
cd $contest_id

for i in {a..z}; do
    upper=$(echo $i | tr '[a-z]' '[A-Z]')
    if [ -e ../../autotaker/$contest_id/$upper/main.rs ]; then
        mv ../../autotaker/$contest_id/$upper/main.rs src/bin/$i.rs
    else
        break
    fi
done

code src/bin/a.rs
