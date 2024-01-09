f5_interval=0.1 # + requesting time

p=$(pwd)
render_template=render-rust.yaml
contest_id=$1
if [ -z $contest_id ]; then
  echo "Usage: prepare-kn.sh <contest_id>"
  exit 1
fi

cd $p/compete
if [ ! -d $contest_id ]; then
  error=$(cargo compete new $contest_id 2>&1)
  if [ $? -ne 0 ]; then
    last_line=$(echo "$error" | tail -n 1)
    start_time=$(echo "$last_line" | sed -e 's/.*will begin at //g')
    echo "start time: $start_time"

    # コンテストまで 1 秒前まで待ちながら、 10 秒おき(時間を常に確認して秒数%10 == 0 だったら)に今の時間とコンテスト開始時間を表示する
    while [ $(date +%s) -lt $(($(date -d "$start_time" +%s) - 1)) ]; do
      if [ $(($(date +%s) % 10)) -eq 0 ]; then
        echo "now: $(date +%H:%M:%S), start: $start_time"
      fi
      sleep 1
    done
    cargo compete new $contest_id
    while [ $? -ne 0 ]; do
      sleep $f5_interval
      cargo compete new $contest_id
    done
  fi
fi

for ((i = 0; i < 26; i++)); do
  clr[$i]=1
done

cd $p/atcoder-tools
if [ ! -d ../compete/$contest_id ]; then
  atcoder-tools gen $contest_id --lang rust --workspace . --template template/template.rs
else
  echo -n "clear source files? [y/n/!abcd to clear except a, b, c, d]: "
  read answer

  if [ $answer = "y" ] || [ $answer = "Y" ]; then
    rm -rf $p/atcoder-tools/$contest_id
    atcoder-tools gen $contest_id --lang rust --workspace . --template template/template.rs
  elif [ "${answer:0:1}" = "!" ]; then
    for ((i = 1; i < ${#answer}; i++)); do
      clr[$(printf "%d" \'${answer:$i:1}) - 97]=0
    done
    rm -rf $p/atcoder-tools/$contest_id
    atcoder-tools gen $contest_id --lang rust --workspace . --template template/template.rs
  else
    for ((i = 0; i < 26; i++)); do
      clr[$i]=0
    done
  fi
fi

launch_json_body="{
  \"version\": \"0.2.0\",
  \"configurations\": [
"

for i in {a..z}; do
  upper=$(echo $i | tr '[a-z]' '[A-Z]')
  if [ -e $p/atcoder-tools/$contest_id/$upper/main.rs ]; then
    if [ ${clr[$(printf "%d" \'$i) - 97]} -eq 1 ]; then
      cp $p/atcoder-tools/$contest_id/$upper/main.rs $p/compete/$contest_id/src/bin/$i.rs
    fi

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
echo -e "$launch_json_body" >$p/compete/$contest_id/.vscode/launch.json

# rm -rf $p/compete/$contest_id/src/bin
# cp -r $p/atcoder-tools/$contest_id $p/compete/$contest_id/src/bin

cd $p/compete/$contest_id
code .
code ./src/bin/a.rs

cd $p