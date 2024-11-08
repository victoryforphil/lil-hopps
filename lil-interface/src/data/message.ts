import useDroneStore from '@/state/drone';
import { useLogStore } from '../state/logstore';
import { decode } from '@msgpack/msgpack';
import useControlStore from '@/state/control';
import { useConnectionStore } from '@/state/connection';

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
	Integer = 'Integer',
	Text = 'Text',
	Boolean = 'Boolean',
	StructType = 'StructType',
	Unknown = 'Unknown',
}

type TopicStore = Map<string, string | boolean | number>;

const topicStore: TopicStore = new Map();

function getCurrentTime(): string {
    const now = new Date();
    const hours = now.getHours().toString().padStart(2, '0');
    const minutes = now.getMinutes().toString().padStart(2, '0');
    const seconds = now.getSeconds().toString().padStart(2, '0');
    return `${hours}:${minutes}:${seconds}`;
}

function removeFirstAndLastCharacter(str: string): string {
    if (str.length <= 2) return '';
    return str.slice(1, -1);
}

function parseDataFields(web_message: WebMessage, topic_store: TopicStore) {
	const timeStamp = getCurrentTime()
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
				case DataType.Integer:
					value = +valueStr;
					break;
				case DataType.Text:
					value = removeFirstAndLastCharacter(valueStr);
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

			if (topic.includes("log/text")) {
				topic_store.set(topic, `[${timeStamp}]: ${value}`)
			} else {
				topic_store.set(topic, value)
			}
			// return { topic, valueType, value };
		} else {
			topic_store.set(topic, datapoint)
		}
	});
}

/**
 * Parses the lil-gcs message data
 * @param data event.data coming from websocket
 */
export function parseWebMessage(data: any) {


	const message = decode(new Uint8Array(data)) as WebMessage;

	console.log(message);

	const currentTimestamp = Date.now() / 1000;
	const latency = (currentTimestamp - message.timestamp) * 1000;

	parseDataFields(message, topicStore);

    if (topicStore.has("log/text")) {
        useLogStore.getState().addLogMessage(topicStore.get("log/text") as string);
    }

	if (topicStore.has("status/health")) {
		console.log(topicStore.get("status/health"))
	}

	useDroneStore.getState().overrideMap(topicStore);
	useConnectionStore.getState().setRecieved(latency); // This is latency but it doesn't ahve to be.

	// console.log(`Latency: ${latency.toFixed(2)} ms`);
	// console.log(`Got ${message.data.length} messages`);
}
