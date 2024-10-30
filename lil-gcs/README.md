# Lil - Ground Control System (GCS)
TBD 

----
```
---- Planning Section ----
```
----

# Stage 1: MVP

# Overview
In order to support Hop #3, which entails using our own Rust stack to command high-level control over an Ardupilot quad, we need an efficient way to view minimal live data, send high-level commands, and perform basic configuration and setup. 

## Requirements
 - Can connect remotely to a Victory Datastore / Command / Pub Sub network over a given medium (TCP over WIFI for Hop #3)
 - Operate w/o a reliable internet connection
 - Web-based dashboard connecting to a native bridge for Victory-suite 
 - View / update key-value parameters pairs from a MavLink quadcopter (See Data / Parameters)
 - View live position of the quadcopter 
	 - **Min:** View position fields like X,Y,Z in NED space or Lat Lon Alt in a data table or large numbers.
	 - **Perferred**: A 3D or 2D view (with optional map for LLA) to view the drones pose 
 - View live messages  / "logs"
	 - Occasionally, status messages will come in, usually on the `log/{}` set of topics. We should display this prominently, these are usually just strings.
	 - **Bonus:** Keep track of these historical for a log view
	 - Command acknowledgments are a part of this. 
 - Send High-level commands
	 - Arm/Disarm 
	 - Current Mode (out of drop down )
		 - Stabilize
		 - Auto - Mission
		 - Auto -  CoProccessor
		 - Return
		 - Break
	 - Takeoff / Land
 - View live status's 
	 - Lil hopps publishes multiple `/status` topics, usually boolean, for various systems like sensors or autopilot health.
	 - **Min:** View these current values with some color / vibrant UX. RED = faling, GREEN = good. 
	 - **Bonus:** Time history
# Design
![image](https://github.com/user-attachments/assets/2066f45d-fd74-4789-bf31-7962b53d166d)

# Data
## Parameters
- Parameters are how Ardupilot (and other MavLink autopilots) store various configurable settings and acts as the main entrypoint to setting up an autopilot.
- These parameters are often updated and need checking before a flight, so easy and quick access is important.
- [Parameter Protocol · MAVLink Developer Guide](https://mavlink.io/en/services/parameter.html)
- Consists of "key / value" pairs 
	- keys are strings, stored in a byte array for their ID./
	- values can be multile numeric types
- MavLink will report these ParameterValues on boot, and thus you will see ~1000 topics under `/params` be loading in near the start of boot.
- These are sent using MavLink `PARAMETER_VALUE` message.
- Can be updated using `PARAMETER_SET` message, datastore will (soon) automatically handle this anytime you submit a parameter change
- **Updating a parameter using DataStore**
	- Set the parameter in DataStore to a new value 
		- IE: `params/serial4_baud` = 9600
	- Set the parameter in DataStore `ack` to false
		- IE: `params/ack = false`
	- Data store will detect this false ack, read the new value, update MavLink, then set ack to true. 
	- **Bonus:** Ideally datastore will detect a new value at a later time and use that for change detection.
### Examples
```csv
params/acro_bal_pitch, Float(1.0)
params/acro_bal_roll,  Float(1.0)
params/acro_rp_expo,   Float(0.30000001192092896)
params/acro_rp_p,      Float(4.5)
params/acro_thr_mid,   Float(0.0)
params/ack,            Bool(false) # Update THIS!
params/acro_trainer,   Float(2.0)
params/acro_y_expo,    Float(0.0)
params/acro_yaw_p,     Float(4.5)
params/adsb_enable,    Float(0.0)
```

## Pose
- Represents location information of the quad
- Prefixed with `/pose`
- `pose/ned` - provided local (North East Down) coordinates of the quad
- `pose/attitude` - provides attitude (in radians) of the quad
- `pose/lla` - provides Lat Long Altitutude of the quad (TBI)
### Related Types
- QuadPoseNED - `lil-link/src/common/types/pose_ned.rs`
	- `position: Vector3
	- `velocity: Vector3`
- Vector3 -  `lil-link/src/common/types/vector3.rs`
	- `x: f64`
	- `y: f64`
	- `z: f64`

### Examples
```cs
pose/attitude/_type,              StructType("QuadAttitude")
pose/attitude/rpy_radians/_type,  StructType("Vector3")
pose/attitude/rpy_radians/x,      Float(-0.005584385246038437)
pose/attitude/rpy_radians/y,      Float(-0.006445697043091059)
pose/attitude/rpy_radians/z,      Float(-1.7597808837890625)
pose/ned/_type,                   StructType("QuadPoseNED")
pose/ned/position/_type,          StructType("Vector3")
pose/ned/position/x,              Float(-0.3316657543182373)
pose/ned/position/y,              Float(-0.3017107844352722)
```
## Status
TBD
### Examples
```cs
status/sensors/reverse_motor,       Boolean(false)
status/sensors/satcom,              Boolean(false)
status/sensors/terrain,             Boolean(true)
status/sensors/vision_position,     Boolean(false)
status/sensors/xy_position_control, Boolean(false)
status/sensors/yaw_position,        Boolean(true)
status/mode/_type,                  StructType("QuadAutopilotStatus")
status/mode/guided_enabled,         Boolean(true)
status/mode/hil_enabled,            Boolean(false)
status/mode/manual_input_enabled,   Boolean(true)
status/mode/safety_armed,           Boolean(true)
status/mode/stabilize_enabled,      Boolean(true)
status/mode/test_enabled,           Boolean(false)
status/system/system,               Text("MAV_STATE_ACTIVE")
```

## Logs
TBD
### Example
```cs
log/text,Text("Frame: QUAD")
log/text,Text("Frame: QUAD")
```


## Commands
TBD
### Example
```cs
cmd/arm/_type,         StructType("ArmMessage")
cmd/arm/ack,           Boolean(false)
cmd/arm/arm,           Boolean(true)
cmd/mode/_type,        StructType("QuadSetModeRequest")
cmd/mode/ack,          Boolean(false)
cmd/mode/mode,         Text("Guided")
cmd/takeoff/_type,     StructType("QuadTakeoffRequest")
cmd/takeoff/ack,       Boolean(false)
cmd/takeoff/height,    Float(11.0)
```
