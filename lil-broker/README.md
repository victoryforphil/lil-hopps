#design-document, #lil-hops
# Overview
In order to unify all data on a UAV system, a central data broker should be created that can store and share all data needed for UAV operations, in a controlled and smart manner. This will allow a single data system to handle log recording, telemetry, and command and control as they all just write/read from the central data store
## Current Short-comings
- Recorded and live data is handled differently, meaning live data can usually be passed through easily, however when it comes to recording it, separate, more involved steps are usually needed.
- Current data / log storage system some-what combines all UAVs into one giant data store, instead we should try for per-uav data stores.
## Goals
- Store key/value pairs at a specific timestamp
- Tag Based system
	- `sys/` tags for system control actions (such as saving behavior, notification, caching, etc. ) 
	- `user/` tags for user defined tagging behavior
	- Tags can also have values, such `sys/save_rate=1000ms` or `user/scenario=test_scenario`
- Cache system
	- Only store values when they have changes (can be override with `sys/store-all` tag)
	- Only store values at a maximum update rate (can be overridden with `sys/store-always`)
- External connections can use this system to write and read given data about a UAV, including incoming commands and ground truth simulation data.
- 

# Implementation - Stage 1
![[Data Broker Drawing.canvas]]
## Data Storage
## DataPoint
 - Stores a specific [[#Primitives]] at a specific time
 - Optionally has a set of tags that will applied to only this entry
	 - Some options may be a specific data key for that run
	 - Acknowledgment tags are applied this way. 
 - Interall Structure
	 - timestamp
	 - value
	 - tags[]
### Buckets
- Store a specific topic across time
- Has their own set of tags that will automatically apply across all topics
	- Such tags may be collection-related tags such as caching behavior (save every..)
	- Tags may be tags to be automatically applied to all results / entries (will be automatically returned / populated on query, not stored)
- Internally made up of a sorted Map
	- `key: Timestamp`
	- `value: DataPoint`
- Handles all querying for a specific topic / pull data out of the topic
## Database

- Store a set of [[#Buckets]] across multiple topic keys
- Internally made up of a map
	- `key: String (Key)`
	- `value Bucket`
## Primitives
- Number (float64)
- String
- Boolean
- String Array
- Boolean Array
- Number Array

## Data Flow (Request Response)
For early iterations of this data broker design, we will use a request/response system, with each "cycle" consisting of a client sending a request for specific data and a response. being met with the data. 

In later iterations, it may be possible to have one request setup further subscriptions, such as requesting to `send_on_change`, in which case, for connection mediums that support sending, the server will automatically start to forward packets without any additional need requests. Although possible, keeping this out of scope for Stage 1 until its proven to be needed.
### Example Response
```
BrokerResponse{
	- my/topic/one
		- data: 1
		- tags: []
		- timestamp: 0000
	- my/topic/two
		- ...
	- meta:
		- is_succesful: true
		- n_results = 2
		- query_time = 10ms
}
```


## Querying
- Queries are sent via a query object (most likely a enum) with optional/defaulted arguments
	- `GET_LATEST` - Get the latest value (time wise) for a given set of topics.
		- `topics[]` (Required): Which topics to retrieve the latest values for
		- `ack_topics[]` (Optional): Which topics to acknowledge
		- `tag_filters[]` (Optional): Tag filter options
	- `LOOKUP` - Get the last values for a set of keys before/after a given timestamp, think most recent valid values up to the timestamp.
		- `topics[]` (Required): Which topics to retrieve
		- `timestamp` (Required): Which timestep to use as a max
		-  `ack_topics[]` (Optional): Which topics to acknowledge 
		- `tag_filters[]` (Optional): Tag filter options
		- `direction` (Defaulted = before): Whether to use last result or next one after a given timestamp for querying.
	- `LOOKUP_RANGE` - Get all the valid values for a set of topics between a given time range
		- `topics[]` (Required): Which topics to retrieve
		- `timestamp_start` (Required): Which timestamp to use as a start of the range
		- `timestamp_end` (Required): Which timestamp to use as the end of the range
		- `ack_topics[]` (Optional): Which topics to acknowledge 
		- `tag_filters[]` (Optional): Tag filter options
	- `WRITE`: (Required): Write a topic
		- `topic` - Topic key to write to
		- `data` - Data to set value to
		- `timestamp` - Timestamp of value
		- `tags` - Tags for value
## Tag Filtering
- Tags are objects consisting of a string key and optionally a value associated with this.
- Basic schema goes:
	- `Tag`
		- `tag_key` - the name of the key 
		- `tag_value` - value of the tag
- Tag filtering works by providing a list of tags to check for and behaviour once found
	- `TagFilter[]`
		- `Tag`
			- `tag_key` - the name of the key  to search
			- `tag_value` - value of the tag to search
		- `behaviour` (one of the options below)
			- `exclude` - Exclude all matching tags w/ matching data. 
			- `include` - Include all matching tags w/ matching data
			- `exclude_ignore_value` - Exclude matching tags, even if the data inside is not matched
			- `include_ignore_value` - Include matching tags, even if the data inside is not matched