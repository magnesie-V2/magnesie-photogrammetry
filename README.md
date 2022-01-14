# Magnesie - photogrammetry


## Build & Run docker container

Note :
+ --privileged option for run command allows to execute turbostat or perf scripts for measuring power consumption

### Production

```sh
# Build
docker build -t photogrammetry .

# Run
docker run --rm --privileged --name=magnesie-instance -it -p 7879:8000 photogrammetry
```

### Development

```sh
# Build
docker build -t photogrammetry-dev --file Dockerfile-dev .

# Run : creating volume to follow webservice change in live
docker run --rm --privileged --name=magnesie-instance-dev -it -p 7879:8000 -v $(pwd)/webservice:/webservice photogrammetry-dev
```

## Architecture

The webservice responds to orchestrator requests to create photogrammetry jobs. (see routes in /webservice/main.rs)

For each job, the webservice launches the script `run.sh` that handles the photogrammetry stack and notifies the orchestrator when finished.
