set -e
docker build -t quay.io/wutiarn/idgen-rs .
docker push quay.io/wutiarn/idgen-rs
