import { ActionIcon, Badge, Button, ScrollArea, Select } from '@mantine/core';
import { IconArrowLeft, IconDots, IconLambda, IconPlugConnected, IconTriangle } from '@tabler/icons-react';
import { BatteryLabel } from './battery';
import clsx from 'clsx';
import useControlStore from '@/state/control';
import { notifications } from '@mantine/notifications';
import { useConnectionStore } from '@/state/connection';
import { useLogStore } from '@/state/logstore';
import { useEffect, useState } from 'react';
import useVictoryValue, { MapValue } from '@/hooks/useVictoryValue';
import { useGCSConnection } from '@/data/ws.singleton';

export function SidebarHeader() {
	const connected = useConnectionStore((state) => state.connected);
	const connecting = useConnectionStore((state) => state.connecting);
	const latency = useConnectionStore((state) => state.lastRecieved);

	return (
		<div className="flex flex-1 justify-between items-center">
			<div className="flex items-center">
				<ActionIcon variant="subtle" aria-label="Back to overview" color="gray" className="mr-2">
					<IconArrowLeft style={{ width: '70%', height: '70%' }} stroke={1.5} />
				</ActionIcon>
				<div className="flex gap-2 items-center">
					<div>Drone</div>
					<div className="text-xs font-mono opacity-50 pt-2">v.0.0.1</div>
				</div>
				<Badge color={connected ? 'green' : 'red'} size="xs" className="ml-4">
					{connected ? 'Live' : 'Disconnected'}
				</Badge>
				<div className="text-xs font-light font-mono ml-2 opacity-50">{latency.toFixed(2)}ms</div>
			</div>

			<div className="flex gap-2">
				<ActionIcon
					variant="filled"
					aria-label="Options"
					color={connected ? 'red' : 'green'}
					loading={connecting}
					onClick={() => {
						if (connected) {
							// TODO: Disconnect? Why
						} else {
							useGCSConnection().restart();
						}
					}}
				>
					<IconPlugConnected style={{ width: '70%', height: '70%' }} stroke={1.5} />
				</ActionIcon>
				<ActionIcon variant="filled" aria-label="Options" color="gray">
					<IconDots style={{ width: '70%', height: '70%' }} stroke={1.5} />
				</ActionIcon>
			</div>
		</div>
	);
}

const Modes = ['Stabilize', 'Acro', 'AltHold', 'Auto', 'Guided', 'Loiter', 'Return', 'Land', 'PosHold', 'Brake', 'Follow'];

// TODO: arm buttons here. For the cool looks.
export function DroneLabel(props: { name: string; battery: number }) {
	const [modeSelect, setModeSelect] = useState<string | null>('');
	const [battery_remaining] = useVictoryValue('status/battery');
	const [mode] = useVictoryValue('cmd/mode/mode');

	useEffect(() => {
		if (mode) {
			setModeSelect(mode as string);
			notifications.show({
				title: 'Mode Changed',
				message: `Updated Mode: ${mode}`,
			});
			console.log(mode);
		}
	}, [mode]);

	const setMode = (newMode: string | null) => {
		useGCSConnection().sendMessage(`MODE:${newMode}`);
		setModeSelect(newMode);
	};

	return (
		<div className="flex flex-col rounded-lg info-container">
			<div className="flex flex-1 justify-between items-center">
				<div className="flex items-center font-semibold">{props.name}</div>
				<Select
					variant="default"
					placeholder="Mode"
					value={modeSelect}
					onChange={setMode}
					data={Modes}
					allowDeselect={false}
				/>
				<div>
					<BatteryLabel charge={(battery_remaining as number) ?? 0} />
				</div>
			</div>
		</div>
	);
}

export function PositionContainer() {
	const [pose_x] = useVictoryValue('pose/ned/position/x');
	const [pose_y] = useVictoryValue('pose/ned/position/y');
	const [pose_z] = useVictoryValue('pose/ned/position/z');

	const getValue = (value: MapValue | undefined) => {
		if (value === undefined) {
			return 'N/A';
		} else {
			const val = value as number;
			return val.toFixed(1);
		}
	};

	return (
		<div className="info-container flex flex-col rounded-lg">
			<div className="w-fit text-sm font-light opacity-70">Position</div>
			<div className="flex w-full flex-wrap justify-between mt-2">
				<div className="w-[30%] flex flex-col">
					<div className="text-4xl font-mono font-black">{getValue(pose_x)}</div>
					<div className="font-mono opacity-60">X</div>
				</div>
				<div className="w-[30%] flex flex-col">
					<div className="text-4xl font-mono font-black">{getValue(pose_y)}</div>
					<div className="font-mono opacity-60">Y</div>
				</div>
				<div className="w-[30%] flex flex-col">
					<div className="text-4xl font-mono font-black">{getValue(pose_z)}</div>
					<div className="font-mono opacity-60">Z</div>
				</div>
			</div>
		</div>
	);
}

function radToDeg(radians: number): number {
    return radians * (180 / Math.PI);
}

export function AttitudeContainer() {
	const [pose_x] = useVictoryValue('pose/attitude/rpy_radians/x');
	const [pose_y] = useVictoryValue('pose/attitude/rpy_radians/y');
	const [pose_z] = useVictoryValue('pose/attitude/rpy_radians/z');

	const [x_deg, setX_deg] = useState<number | undefined>(null!);
	const [y_deg, setY_deg] = useState<number | undefined>(null!);
	const [z_deg, setZ_deg] = useState<number | undefined>(null!);

	useEffect(() => {
		if (pose_x) {
			setX_deg(radToDeg(pose_x as number));
		}
	}, [pose_x])
	
	useEffect(() => {
		if (pose_y) {
			setY_deg(radToDeg(pose_y as number));
		}
	}, [pose_y])

	useEffect(() => {
		if (pose_z) {
			setZ_deg(radToDeg(pose_z as number));
		}
	}, [pose_z])

	const getValue = (value: MapValue | undefined) => {
		if (value === undefined) {
			return 'N/A';
		} else {
			const val = value as number;
			return val.toFixed(1);
		}
	};

	return (
		<div className="info-container flex flex-col rounded-lg">
			<div className="w-fit text-sm font-light opacity-70">Attitude</div>
			<div className="flex w-full flex-wrap justify-between mt-2">
				<div className="w-[30%] flex flex-col">
					<div className="text-4xl font-mono font-black">{getValue(x_deg)}</div>
					<div className="font-mono opacity-60">Roll</div>
				</div>
				<div className="w-[30%] flex flex-col">
					<div className="text-4xl font-mono font-black">{getValue(y_deg)}</div>
					<div className="font-mono opacity-60">Pitch</div>
				</div>
				<div className="w-[30%] flex flex-col">
					<div className="text-4xl font-mono font-black">{getValue(z_deg)}</div>
					<div className="font-mono opacity-60">Yaw</div>
				</div>
			</div>
		</div>
	);
}

export function StatusContainer() {
	// subscribe to `status/sensors/gps`
	const [gps_status] = useVictoryValue('status/sensors/gps');
	const [satcom] = useVictoryValue('status/sensors/satcom');
	const [terrain] = useVictoryValue('status/sensors/terrain');
	const [vision_position] = useVictoryValue('status/sensors/vision_position');
	const [xy_position_control] = useVictoryValue('status/sensors/xy_position_control');
	const [yaw_position] = useVictoryValue('status/sensors/yaw_position');
	const [guided_enabled] = useVictoryValue('status/mode/guided_enabled');
	const [hil_enabled] = useVictoryValue('status/mode/hil_enabled');
	const [manual_input_enabled] = useVictoryValue('status/mode/manual_input_enabled');
	const [safety_armed] = useVictoryValue('status/mode/safety_armed');
	const [stabilize_enabled] = useVictoryValue('status/mode/stabilize_enabled');
	const [test_enabled] = useVictoryValue('status/mode/test_enabled');
	// const [mav_state] = useVictoryValue('status/system/system'); // This is making it look guly for now.

	// Needs a special case.
	// status/system/system, Text("MAV_STATE_ACTIVE")

	return (
		<div className="info-container flex flex-col rounded-lg">
			<div className="w-fit text-sm font-light opacity-70">Systems Status</div>
			<div className="flex w-full flex-wrap justify-center">
				{/* <div className="flex flex-col w-[40%]">{<StatusLabel name={"MAV Status"} status={mav_state as string} />}</div> */}
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'GPS'} status={gps_status as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Sat Com'} status={satcom as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Terrain'} status={terrain as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Vision Pos'} status={vision_position as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'XY Control'} status={xy_position_control as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Yaw Control'} status={yaw_position as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Guided'} status={guided_enabled as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'HIL'} status={hil_enabled as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Manual Input'} status={manual_input_enabled as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Safety Armed'} status={safety_armed as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Stabilize'} status={stabilize_enabled as boolean} />}
				</div>
				<div className="flex flex-col w-[40%]">
					{<BoolStatusLabel name={'Test Mode'} status={test_enabled as boolean} />}
				</div>
			</div>
		</div>
	);
}

function BoolStatusLabel(props: { name: string; status: boolean }) {
	const getStatus = () => {
		if (props.status) {
			return <div className="text-green-400">Yes</div>;
		} else {
			return <div className="text-red-400">No</div>;
		}
	};

	return (
		<div className="flex w-fit p-1">
			<div>{props.name}:</div>
			<div className="font-bold ml-2">{getStatus()}</div>
		</div>
	);
}

export function LogBox() {
	const [reversedList, setReversedList] = useState<string[]>([]);
	const log_message = useLogStore((state) => state.log_messages);

	useEffect(() => {
		setReversedList(log_message.reverse());
	}, [log_message]);

	// TODO: undo code spaghetti from bad formatter early on.
	return (
		<div className="info-container flex flex-col rounded-lg">
			<div className="w-fit text-sm font-light opacity-70">Drone Logs</div>
			<ScrollArea h={200}>
				<div className="flex flex-col mt-2 font-mono font-bold text-slate-300 text-xs">
					{reversedList.map((l, i) => {
						if (i == 0) {
							return (
								<div className="flex items-center" key={i}>
									<IconLambda width={'0.9rem'} />
									<div key={i} className={clsx('w-fit ml-1', { 'opacity-70': i != 0 })}>
										{l}
									</div>
								</div>
							);
						}
						return (
							<div key={i} className={clsx('w-fit', { 'opacity-70': i != 0 })}>
								{l}
							</div>
						);
					})}
				</div>
			</ScrollArea>
		</div>
	);
}

export function ArmButtons() {
	const { armed, toggleArm, flying, toggleFlying } = useControlStore();

	return (
		<div className="flex justify-between">
			<Button
				color={armed ? 'red' : 'green'}
				size="lg"
				w={'45%'}
				variant="filled"
				onClick={() => {
					toggleArm();

					if (!armed) {
						notifications.show({
							title: 'Control System',
							message: 'Arming Drone',
							color: 'red',
						});
						useGCSConnection().sendMessage('ARM');
					} else {
						useGCSConnection().sendMessage('DISARM');
						if (flying) toggleFlying();
					}
				}}
			>
				{armed ? 'Disarm' : 'Arm'}
			</Button>
			<Button
				color={armed ? (flying ? 'red' : 'green') : 'gray'}
				variant="outline"
				size="lg"
				w={'45%'}
				disabled={!armed}
				onClick={() => {
					toggleFlying();

					if (!flying) {
						notifications.show({
							title: 'Control System',
							message: 'Taking off',
						});
						useGCSConnection().sendMessage('TAKEOFF');
					} else {
						useGCSConnection().sendMessage('LAND');
					}
				}}
			>
				{flying ? 'Land' : 'Take Off'}
			</Button>
		</div>
	);
}

export function NoDrone() {
	const connecting = useConnectionStore((state) => state.connecting);

	return (
		<div className="flex min-h-[50svh] items-center justify-center flex-col gap-5">
			<IconTriangle color="red" />
			<div className="font-bold">No Ground Station is connected</div>
			<Button
				onClick={() => {
					useGCSConnection().restart();
				}}
				variant="default"
				loading={connecting}
			>
				<IconPlugConnected size={20} className="mr-2" />
				Reconnect
			</Button>
		</div>
	);
}
