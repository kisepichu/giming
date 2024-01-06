# usage: source init.sh <contest_id>

contest_id=$1
if [ -z $contest_id ]; then
    echo "Usage: init.sh <contest_id>"
    return 1
fi

source ./env.sh
source ./prepare-kn.sh $contest_id
