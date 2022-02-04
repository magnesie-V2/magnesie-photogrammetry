# Magnesie - photogrammetry

## Work with Grid'5000 node

### Prerequisite
+ Grid'5000 account
+ SSH keys : write pub key on Grid'5000 account settings

### Commands
```sh
# Connect to Grid'5000 front (use ssh private key)
ssh <login>@access.grid5000.fr

# Go to a Grid'5000 site
ssh nantes

# Reserve a Grid'5000 node
oarsub -I
# (It generates a job_id that you can use to access to the node from another terminal : $ oarsub -C <job_id>)
# (Note the reserved node ID for next step)

# Install docker on node
g5k-setup-docker

### Install and launch your webservice...

# Port forward to access to webservice from your local machine
ssh <login>@access.grid5000.fr -L <local-port>:<g5k-node>.<site>.grid5000.fr:<remote-port>

```

## Build & Run docker container

Note :
+ --privileged option for run command allows to execute turbostat or perf scripts for measuring power consumption

### Production

#### From GHCR
```sh
# Build & Run
docker run --rm --privileged instance -it -p 7879:8000 ghcr.io/magnesie-v2/photogrammetry
```

#### From source code
```sh
# Build
docker build -t photogrammetry .

# Run
docker run --rm --privileged --name=magnesie-instance -it -p 7879:8000 photogrammetry
```

### Development

```sh
# Build
docker build -t photogrammetry-dev --file dev.Dockerfile .

# Run : creating volume to follow webservice change in live
docker run --rm --privileged --name=magnesie-instance-dev -it -p 7879:8000 -v $(pwd)/webservice:/webservice photogrammetry-dev
```

## Architecture

The webservice responds to orchestrator requests to create photogrammetry jobs. (see routes in /webservice/main.rs)

For each job, the webservice launches the script `run.sh` that handles the photogrammetry stack and notifies the orchestrator when finished.e
