# Magnes.ie - photogrammetry

## Build docker image

```sh
docker build --tag magnesie-photogrammetry . --no-cache
```

## Run docker container

```sh
docker run --rm --name=magnesie-instance -i -t -p 7879:8000 magnesie-photogrammetry
```

## Architecture

The webservice responds to orchestrator requests to create photogrammetry jobs. (see routes in /webservice/main.rs)

For each job, the webservice launches the script `run.sh` that handles the photogrammetry stack and notifies the orchestrator when finished.
