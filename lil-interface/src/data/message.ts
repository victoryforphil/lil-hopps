import { decode } from "@msgpack/msgpack";

/**
 * Parses the lil-gcs message data
 * @param data event.data coming from websocket
 */
export function parseWebMessage(data: any) {

    const message = decode(new Uint8Array(data)) as {
        timestamp: number;
        data: { topic: string; datapoint: string }[];
    };
    
    // Get current timestamp in the same unit (e.g., nanoseconds if sent in nanos)
    const currentTimestamp = Date.now() * 1e6; // Convert to nanoseconds (assuming Rust timestamp is in nanos)
    
    // Calculate latency in milliseconds
    const latency = (currentTimestamp - message.timestamp) / 1e6; // Convert nanoseconds to milliseconds
    
    console.log(`Latency: ${latency.toFixed(2)} ms`);
    // console.log(`Received data at ${new Date(message.timestamp / 1e6).toISOString()}:`);
    
    // message.data.forEach(({ topic, datapoint }) => {
    //     console.log(`Topic: ${topic}, Datapoint: ${datapoint}`);
    // });

    console.log(`Got ${message.data.length} messages`)
}

