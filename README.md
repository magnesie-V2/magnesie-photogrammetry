# Magnes.ie - photogrammetry

## Build docker image
```sh
#For release
docker build --tag magnesie-photogrammetry .

#For dev
docker build --tag magnesie-photogrammetry-dev --file ./Dockerfile-dev .
```

## Run docker container
```sh
#start a new container <replace paths>

#For release
docker run --rm --name=magnesie-instance -i -t -p 80:8000 magnesie-photogrammetry

#For dev
docker run --rm --name=magnesie-instance-dev -i -t -v $(pwd)/webservice:/webservice -p 80:8000 magnesie-photogrammetry-dev
```