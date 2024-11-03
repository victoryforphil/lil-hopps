import { ActionIcon, Badge, Button, ScrollArea } from '@mantine/core';
import {
	IconArrowLeft,
	IconDots,
	IconLambda,
	IconPlugConnected,
} from '@tabler/icons-react';
import { BatteryLabel } from './battery';
import clsx from 'clsx';
import useControlStore from '@/state/control';
import { notifications } from '@mantine/notifications';

export function SidebarHeader() {
	return (
		<div className="flex flex-1 justify-between items-center">
			<div className="flex items-center">
				<ActionIcon
					variant="subtle"
					aria-label="Back to overview"
					color="gray"
					className="mr-2"
				>
					<IconArrowLeft
						style={{ width: '70%', height: '70%' }}
						stroke={1.5}
					/>
				</ActionIcon>
				<div className="flex gap-2 items-center">
					<div>Drone</div>
					<div className="text-xs font-mono opacity-50 pt-2">
						v.0.0.1
					</div>
				</div>
				<Badge color="green" size="xs" className="ml-4">
					Live
				</Badge>
			</div>

			<div className="flex gap-2">
				<ActionIcon variant="filled" aria-label="Options" color="red">
					<IconPlugConnected
						style={{ width: '70%', height: '70%' }}
						stroke={1.5}
					/>
				</ActionIcon>
				<ActionIcon variant="filled" aria-label="Options" color="gray">
					<IconDots
						style={{ width: '70%', height: '70%' }}
						stroke={1.5}
					/>
				</ActionIcon>
			</div>
		</div>
	);
}

// TODO: arm buttons here. For the cool looks.
export function DroneLabel(props: { name: string; battery: number }) {
	return (
		<div className="flex flex-col rounded-lg info-container">
			<div className="flex flex-1 justify-between">
				<div className="flex items-center font-semibold">
					{props.name}
				</div>
				<div>
					<BatteryLabel charge={props.battery} />
				</div>
			</div>
			{/* <div>
                <ArmButtons />
            </div> */}
		</div>
	);
}

// TODO: It would be dope if we could specifically sorround sensor values in a data container with a name ...

export function StatusContainer(props: {
	status: { name: string; status: string }[];
}) {
	const getStatusLabels = () => {
		return props.status.map((s, i) => {
			return <StatusLabel name={s.name} status={s.status} key={i} />;
		});
	};

	return (
		<div className="info-container flex flex-col rounded-lg">
			<div className="w-fit text-sm font-light opacity-70">
				Systems Status
			</div>
			<div className="flex flex-col mt-2">{getStatusLabels()}</div>
		</div>
	);
}

function StatusLabel(props: { name: string; status: string }) {
	const getStatus = () => {
		if (props.status.toLowerCase() == 'healthy') {
			return <div className="text-green-400">Healthy</div>;
		} else if (props.status.toLowerCase() == 'offline') {
			return <div className="text-red-400">Offline</div>;
		} else {
			return <div>{props.status}</div>;
		}
	};

	return (
		<div className="flex w-fit p-1">
			<div>{props.name}:</div>
			<div className="font-bold ml-2">{getStatus()}</div>
		</div>
	);
}

const fake_logs: string[] = [
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:01 Initial Log Message',
	'12:03 IMU Boot up',
	'12:04 IMU Borked',
];

export function LogBox() {
	// TODO: ue the logs state variable to display all of them

	return (
		<div className="info-container flex flex-col rounded-lg">
			<div className="w-fit text-sm font-light opacity-70">
				Drone Logs
			</div>
			<ScrollArea h={200}>
				<div className="flex flex-col mt-2 font-mono font-bold text-slate-300">
					{fake_logs.reverse().map((l, i) => {
						if (i == 0) {
							return (
								<div className="flex">
									<IconLambda width={'1rem'} />
									<div
										key={i}
										className={clsx('w-fit ml-2', {
											'opacity-70': i != 0,
										})}
									>
										{l}
									</div>
								</div>
							);
						}
						return (
							<div
								key={i}
								className={clsx('w-fit', {
									'opacity-70': i != 0,
								})}
							>
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
	// States for arming and take off ideally.
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
                            title: "Control System",
                            message: "Arming Drone",
                            color: "red"
                        })
                    } else {
                        toggleFlying();
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
                            title: "Control System",
                            message: "Taking off"
                        })
                    }
				}}
			>
				{flying ? 'Land' : 'Take Off'}
			</Button>
		</div>
	);
}
