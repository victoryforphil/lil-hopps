import { decode, decodeMulti } from '@msgpack/msgpack';
import { parseWebMessage } from '../data/message';
import { useEffect, useState } from 'react';

export const useWebSocket = (url: string) => {
	const [socket, setSocket] = useState<WebSocket | null>(null);
	const [isConnected, setIsConnected] = useState(false);
	const [reconnectAttempts, setReconnectAttempts] = useState(0);

	useEffect(() => {
		let ws: WebSocket | null = null;
		let reconnectTimeout: any;

		const connectWebSocket = () => {
			ws = new WebSocket(url);

			ws.onopen = () => {
				console.log('Connected to WebSocket');
				setIsConnected(true);
				setReconnectAttempts(0); // reset on successful connection

				ws!.binaryType = 'arraybuffer';
			};

			ws.onclose = () => {
				// console.log('WebSocket closed. Attempting to reconnect...');
				// setIsConnected(false);
				// if (ws) {
				//   ws = null;
				//   attemptReconnect();
				// }
			};

			ws.onerror = (error) => {
				console.error('WebSocket error:', error);
			};

			ws.onmessage = (event) => {
				console.log('Message received from WebSocket:', event.data);

				// parseWebMessage(event.data);
				// console.log(decode(event.data))

				// for (const object of decodeMulti(event.data)) {
				//     console.log(object);
				// }

				const message = decode(event.data) as {
					timestamp: number;
					data: { topic: string; datapoint: string }[];
				};

				console.log(message);

				// Calculate latency in milliseconds
				const currentTimestamp = Date.now() / 1000; // Convert to seconds
				const latency = (currentTimestamp - message.timestamp) * 1000; // Convert to ms

				console.log(`Latency: ${latency.toFixed(2)} ms`);
				console.log(
					`Received data at ${new Date(message.timestamp * 1000).toISOString()}:`
				);
				// message.data.forEach(({ topic, datapoint }) => {
				// 	console.log(`Topic: ${topic}, Datapoint: ${datapoint}`);
				// });

				// handle incoming messages here
			};

			setSocket(ws);
		};

		const attemptReconnect = () => {
			const newAttempts = reconnectAttempts + 1;
			setReconnectAttempts(newAttempts);
			const delay = Math.min(newAttempts * 1000, 5000); // Exponential backoff with a cap at 5s

			reconnectTimeout = setTimeout(() => {
				console.log(`Reconnecting attempt #${newAttempts}`);
				connectWebSocket();
			}, delay);
		};

		connectWebSocket();

		return () => {
			if (ws) ws.close();
			if (reconnectTimeout) clearTimeout(reconnectTimeout);
		};
	}, [url, reconnectAttempts]);

	return { socket, isConnected };
};
