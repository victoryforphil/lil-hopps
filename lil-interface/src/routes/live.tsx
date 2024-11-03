import MapContainer from '@/components/map';
import { DroneLabel, StatusContainer, SidebarHeader, LogBox } from '@/components/sidebar';
import { useWebSocket } from '@/hooks/useWebsocket';
import { useLogStore } from '@/state/logstore';

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
        name: "GPS",
        status: "Healthy"
    },
    {
        name: "Sensor1",
        status: "Offline"
    },
    {
        name: "Sensor2",
        status: "Healthy"
    },
    {
        name: "GPS State",
        status: "4"
    }
]

export default function Live() {
	const { isConnected } = useWebSocket('ws://localhost:3030');
	const log_message = useLogStore((state) => state.log_messages);

	return (
		<div className="full-width-container">
			<div className="sidebar">

                <div className='flex flex-col gap-4'> 
                    <SidebarHeader />
                    <DroneLabel name='lil-hopper 01' battery={40} />
                    <StatusContainer status={fake_status_systems} />
                    <LogBox />
                </div>

			</div>
            
			<div className="map-container">
                <MapContainer/>
            </div>
		</div>
	);
}
