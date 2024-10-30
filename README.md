# lil-hopps 
Small rust-powered quadcopter stack. (Visualizer, Simulation, Firmware, Messaging)

## Project Layout   
- lil-broker: Framework for time-series data storage and querying
- lil-hopps: Quadcopter code
- lil-sym: Simulation code w/ vizualizer
- lil-helper: Utility / framework code.

## Quick Luanch of current SIL + GCS
### Launch Ardupilot Docker Container:
```
docker run -it --rm -p 5760:5760 radarku/ardupilot-sitl
```

### Run Quad SIL from the `lil-launcher` crate (TCP Server)
```
cargo run --bin quad_sil -- -c tcpout:localhost:5760 -a 5 
                                  ^ Ardupilot SIL     ^ How many secs to arm
```

### Run the lil-gcs crate (TCP client)
```
cargo run --bin lil-gcs 
```