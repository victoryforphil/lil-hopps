import LargeContentView from '@/components/largeView';
import {
	DroneLabel,
	StatusContainer,
	SidebarHeader,
	LogBox,
	ArmButtons,
	NoDrone,
	PositionContainer,
	AttitudeContainer,
} from '@/components/sidebar';
import { useConnectionStore } from '@/state/connection';

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
				<LargeContentView />
			</div>
		</div>
	);
}

function DroneConnectedView() {
	return (
		<div className="full-width-container">
			<div className="sidebar">
				<div className="flex flex-col justify-between h-full">
					<div className="flex flex-col gap-2">
						<SidebarHeader />
						<DroneLabel name="lil-hopper 01" battery={40} />
						<ArmButtons />
						<StatusContainer />
						<PositionContainer />
						<AttitudeContainer />
					</div>
					<div>
						<LogBox />
					</div>
				</div>
			</div>

			<div className="map-container">
				<LargeContentView />
			</div>
		</div>
	);
}
