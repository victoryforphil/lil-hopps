# lil-hopps 
Small rust-powered quadcopter stack. (Visualizer, Simulation, Firmware, Messaging)

## CI Status

[![Docker Image Build Check](https://github.com/victoryforphil/lil-hopps/actions/workflows/docker.yaml/badge.svg)](https://github.com/victoryforphil/lil-hopps/actions/workflows/docker.yaml)

[![Rust](https://github.com/victoryforphil/lil-hopps/actions/workflows/rust.yaml/badge.svg)](https://github.com/victoryforphil/lil-hopps/actions/workflows/rust.yaml)

[![Web Interface Build](https://github.com/victoryforphil/lil-hopps/actions/workflows/web.yaml/badge.svg)](https://github.com/victoryforphil/lil-hopps/actions/workflows/web.yaml)


## Project Layout   
- lil-broker: Framework for time-series data storage and querying
- lil-hopps: Quadcopter code
- lil-sym: Simulation code w/ vizualizer
- lil-helper: Utility / framework code.

## Quick Luanch of current SIL + GCS
### Launch Ardupilot Docker Container:
```bash
docker run -it --rm -p 5760:5760 radarku/ardupilot-sitl
```

or 

This will always keep it running and run it in the correct mode for ARM
```bash
docker compose up -d
```

### Run Quad SIL from the `lil-launcher` crate (TCP Server)
```bash
cargo run --bin quad_sil -- -c tcpout:localhost:5760 -a 5 
                                  ^ Ardupilot SIL     ^ How many secs to arm
```

### Run the lil-gcs crate (TCP client)
```bash
cargo run --bin lil-gcs 
```


## Hardware Deploy
### Build using dev container + corss

1. Build using docker compose
```
docker-compose build
```
2. Launch using compose
```
docker-compose -up -d
```
3. Attach to rust dev container
```
docker-compose attach rust-dev
```

4. Build
```
cross build --release --bins --target arm-unknown-linux-gnueabihf
```

### Deploy to RPI
```
ansible-playbook -i ansible/inventory.ini ansible/playbooks/deploy_quad_idle.yml
```
