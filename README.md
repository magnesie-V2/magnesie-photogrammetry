# Magnes.ie - photogrammetry

## Build docker image
```sh
#For release
docker build --tag magnesie-photogrammetry-mock .
```

## Run docker container
```sh
#start a new container

docker run --rm --name=magnesie-photogrammetry-mock -i -t -p 7879:8000 -v $(pwd)/ref:/res magnesie-photogrammetry-mock
```
