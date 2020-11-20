# Magnes.ie - photogrammetry

## Build docker image
```sh
docker build --tag magnesie-photogrammetry .
```

## Run docker container
```sh
#start a new container <replace paths>
docker run --name=magnesie-instance -i -t -v <path_to_local_datasets>:/datasets -v <path_to_result_directory>:/res -p 80:8000 magnesie-photogrammetry

#run process on a dataset
sh run.sh dataset_name
```