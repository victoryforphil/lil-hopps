import MapContainer from '@/components/map';
import {
	DroneLabel,
	StatusContainer,
	SidebarHeader,
	LogBox,
	ArmButtons,
    NoDrone,
} from '@/components/sidebar';
import { useWebSocket } from '@/hooks/useWebsocket';
import { useConnectionStore } from '@/state/connection';
import { useLogStore } from '@/state/logstore';
import { useEffect } from 'react';

/**
 * 
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
 */

// Need a section that will tell me if WS is connected. And need a way to reconnect.

const fake_status_systems = [
	{
		name: 'GPS',
		status: 'Healthy',
	},
	{
		name: 'Sensor1',
		status: 'Offline',
	},
	{
		name: 'Sensor2',
		status: 'Healthy',
	},
	{
		name: 'GPS State',
		status: '4',
	},
];

// TODO: Seperate out top and bottom sidebar. So we can have arming always be on the bottom.

export default function Live() {
	const { isConnected, reconnect } = useWebSocket('ws://localhost:3030');
    
	const setConnected = useConnectionStore((state) => state.setConnected);
	const connected = useConnectionStore((state) => state.connected);

	// useEffect(() => {
	// 	// Tell everyone else while we are at it.
	// 	// setConnected(isConnected);
	// }, [isConnected]);


    if (connected) {
        return (
            <DroneConnectedView />
        )
    } else {
        return (
            <NoDroneView reconnect_cb={reconnect} />
        )
    } 
}

function NoDroneView(props: { reconnect_cb: () => void }) {
	return (
		<div className="full-width-container">
			<div className="sidebar">
				<div className="flex flex-col justify-between h-full">
					<div className="flex flex-col gap-4">
						<SidebarHeader reconnect_cb={props.reconnect_cb} />
                        <NoDrone reconnect_cb={props.reconnect_cb} />
					</div>
				</div>
			</div>

			<div className="map-container">
				<MapContainer />
			</div>
		</div>
	);
}

function DroneConnectedView() {
	return (
		<div className="full-width-container">
			<div className="sidebar">
				<div className="flex flex-col justify-between h-full">
					<div className="flex flex-col gap-4">
						<SidebarHeader reconnect_cb={() => {}} />
						<DroneLabel name="lil-hopper 01" battery={40} />
						<ArmButtons />
						<StatusContainer status={fake_status_systems} />
					</div>
					<div>
						<LogBox />
					</div>
				</div>
			</div>

			<div className="map-container">
				<MapContainer />
			</div>
		</div>
	);
}
