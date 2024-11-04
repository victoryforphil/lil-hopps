import MapContainer from '@/components/map';
import {
	DroneLabel,
	StatusContainer,
	SidebarHeader,
	LogBox,
	ArmButtons,
	NoDrone,
} from '@/components/sidebar';
import { useConnectionStore } from '@/state/connection';

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

export default function Live() {
	const connected = useConnectionStore((state) => state.connected);

	if (connected) {
		return <DroneConnectedView />;
	} else {
		return <NoDroneView />;
	}
}

function NoDroneView() {
	return (
		<div className="full-width-container">
			<div className="sidebar">
				<div className="flex flex-col justify-between h-full">
					<div className="flex flex-col gap-4">
						<SidebarHeader />
						<NoDrone />
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
						<SidebarHeader />
						<DroneLabel name="lil-hopper 01" battery={40} />
						<ArmButtons />
						<StatusContainer />
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
