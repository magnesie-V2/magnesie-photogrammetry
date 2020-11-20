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
docker run --name=magnesie-instance -i -t -v <path_to_local_datasets>:/datasets -v <path_to_result_directory>:/res -p 80:8000 magnesie-photogrammetry

#For dev
docker run --name=magnesie-instance-dev -i -t -v <path_to_local_datasets>:/datasets -v <path_to_result_directory>:/res -v $(pwd)/webservice:/webservice -p 80:8000 magnesie-photogrammetry-dev
```