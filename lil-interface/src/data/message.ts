import { useLogStore } from '../state/logstore';
import { decode } from '@msgpack/msgpack';

interface ParsedData {
	// topic: string;
	valueType: DataType;
	value: string | boolean | number;
}

interface WebMessage {
	timestamp: number;
	data: { topic: string; datapoint: string }[];
}

enum DataType {
	Float = 'Float',
	Text = 'Text',
	Boolean = 'Boolean',
	StructType = 'StructType',
	Unknown = 'Unknown',
}

type TopicStore = Map<string, ParsedData>;

const topicStore: TopicStore = new Map();

function parseDataFields(web_message: WebMessage, topic_store: TopicStore) {
	return web_message.data.map(({ topic, datapoint }) => {
		const typeMatch = datapoint.match(/(\w+)\((.*)\)/);

		if (typeMatch) {
			const valueType = typeMatch[1] as DataType;
			const valueStr = typeMatch[2];
			let value: number | string | boolean;

			switch (valueType) {
				case DataType.Float:
					value = parseFloat(valueStr);
					break;
				case DataType.Text:
					value = valueStr;
					break;
				case DataType.Boolean:
					value = valueStr === 'true';
					break;
				case DataType.StructType:
					value = valueStr;
					break;
				default:
					console.log(`Unknown Type: ${typeMatch[1]}`);
					value = valueStr;
					break;
			}

            // Insert or update topic already in store.
            topic_store.set(topic, { valueType, value })
			// return { topic, valueType, value };
		}

        // Insert or update topic already in store.
        topic_store.set(topic, { valueType: DataType.Unknown, value: datapoint })
		// return { topic, valueType: DataType.Unknown, value: datapoint };
	});
}

/**
 * Parses the lil-gcs message data
 * @param data event.data coming from websocket
 */
export function parseWebMessage(data: any) {


	const message = decode(new Uint8Array(data)) as WebMessage;

	const currentTimestamp = Date.now() / 1000;
	const latency = (currentTimestamp - message.timestamp) * 1000;

	// for (const msg of message.data) {
	// 	console.log(`${msg.topic} : ${msg.datapoint}`);
	// }

	parseDataFields(message, topicStore);

	console.log(topicStore);

    if (topicStore.has("log/text")) {
        useLogStore.getState().setLogMessage(topicStore.get("log/text")!.value as string);
    }

	console.log(`Latency: ${latency.toFixed(2)} ms`);
	console.log(`Got ${message.data.length} messages`);
}
