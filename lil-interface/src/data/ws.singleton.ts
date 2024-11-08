import { useConnectionStore } from '@/state/connection';
import { notifications } from '@mantine/notifications';
import { parseWebMessage } from './message';
import useDroneStore from '@/state/drone';
import { useLogStore } from '@/state/logstore';

const GCS_URL = 'ws://localhost:3030';

export function useGCSConnection() {
    return WebSocketSingleton.getInstance();
}

export class WebSocketSingleton {
	private static instance: WebSocketSingleton;
	private webSocket: WebSocket | null;

	private constructor() {
		this.webSocket = new WebSocket(GCS_URL);
		this.setupWebSocket();
	}

	public static getInstance(): WebSocketSingleton {
		if (!WebSocketSingleton.instance) {
			WebSocketSingleton.instance = new WebSocketSingleton();
		}
		return WebSocketSingleton.instance;
	}

	public sendMessage(message: string) {
		if (this.webSocket && this.webSocket.readyState === WebSocket.OPEN) {
			this.webSocket.send(message);
		} else {
			console.warn('WebSocket is not open. Message not sent.');
		}
	}

	private setupWebSocket() {
        useConnectionStore.getState().setConnecting(true);
		if (this.webSocket) {
            this.webSocket!.binaryType = 'arraybuffer';
			this.webSocket.onopen = () => {
				notifications.show({
					title: 'GCS',
					message: 'Websocket connected!',
					color: 'green',
				});
				useConnectionStore.getState().setConnected(true);
                useConnectionStore.getState().setConnecting(false);

				this.webSocket?.send("NEW_CLIENT")
			};

			this.webSocket.onmessage = (event) => {
				this.onMessage(event);
			};

			this.webSocket.onerror = (error) => {
				console.error('WebSocket error:', error);

				useDroneStore.getState().reset();
				useLogStore.getState().reset();
			};

			this.webSocket.onclose = () => {
				notifications.show({
					title: 'GCS',
					message: 'Websocket disconnected!',
					color: 'red',
				});

				useDroneStore.getState().reset();
				useLogStore.getState().reset();

				useConnectionStore.getState().setConnected(false);
                useConnectionStore.getState().setConnecting(false);
			};
		}
	}

	private onMessage(event: any) {
        parseWebMessage(event.data)
	}

    /**
     * Restarts the websocket via the button if possible.
     */
	public restart() {
		if (this.webSocket) {
			this.webSocket.close();
		}
		this.webSocket = new WebSocket(GCS_URL);
		this.setupWebSocket();
	}
}
