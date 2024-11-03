import MapContainer from '@/components/map';
import SidebarHeader from '@/components/sidebar';
import { useWebSocket } from '@/hooks/useWebsocket';
import { useLogStore } from '@/state/logstore';


// Need a section that will tell me if WS is connected. And need a way to reconnect.

export default function Live() {
	const { isConnected } = useWebSocket('ws://localhost:3030');
	const log_message = useLogStore((state) => state.log_messages);

	return (
		<div className="full-width-container">
			<div className="sidebar">
                <SidebarHeader />
				<div className="flex flex-row gap-10 items-center w-full">
					<div>
						{isConnected ? (
							<h1>Connected To Websocket</h1>
						) : (
							<h1>Not Connected</h1>
						)}
					</div>
					<div>{log_message}</div>
				</div>
			</div>
			<div className="map-container">
                <MapContainer/>
            </div>
		</div>
	);
}
