p=$(pwd)
render_template=render-rust.yaml
contest_id=$1
if [ -z $contest_id ]; then
    echo "Usage: init.sh <contest_id>"
    exit 1
fi

cd $p/compete
cargo compete new $contest_id

cd $p/atcoder-tools
if [ ! -d $contest_id ]; then
    atcoder-tools gen $contest_id --lang rust --workspace . --template template/template.rs
fi


# {
#   "version": "0.2.0",
#   "configurations": [
#     { # これを問題数文だけ作る
#       "type": "lldb",
#       "request": "launch",
#       "name": "Debug",
#       "cargo": {
#         "args": [
#           "build",
#           "--bin=<executable file>"
#         ],
#         "filter": {
#           "name": "<executable file>",
#           "kind": "bin"
#         }
#       },
#       "args": [],
#       "cwd": "${workspaceFolder}"
#     }
#   ]
# }

launch_json_body="{
  \"version\": \"0.2.0\",
  \"configurations\": [
"


for i in {a..z}; do
    upper=$(echo $i | tr '[a-z]' '[A-Z]')
    if [ -e $p/atcoder-tools/$contest_id/$upper/main.rs ]; then
        cp $p/atcoder-tools/$contest_id/$upper/main.rs $p/compete/$contest_id/src/bin/$i.rs

        launch_json_body+="    {
      \"type\": \"lldb\",
      \"request\": \"launch\",
      \"name\": \"Debug $i\",
      \"cargo\": {
        \"args\": [
          \"build\",
          \"--bin=$contest_id-$i\"
        ],
        \"filter\": {
          \"name\": \"$contest_id-$i\",
          \"kind\": \"bin\"
        }
      },
      \"args\": [],
      \"cwd\": \"\${workspaceFolder}\"
    },
"
    else
        break
    fi
done

launch_json_body+="  ]
}"

mkdir -p $p/compete/$contest_id/.vscode
echo -e "$launch_json_body" > $p/compete/$contest_id/.vscode/launch.json

# rm -rf $p/compete/$contest_id/src/bin
# cp -r $p/atcoder-tools/$contest_id $p/compete/$contest_id/src/bin

cd $p/compete/$contest_id
code .
code ./src/bin/a.rs
