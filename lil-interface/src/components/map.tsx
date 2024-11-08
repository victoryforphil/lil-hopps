import useVictoryValue from '@/hooks/useVictoryValue';
import { ScatterChart } from '@mantine/charts';
import { Center, rem, SegmentedControl } from '@mantine/core';
import { IconHexagonalPyramid, IconMap, IconTimeline } from '@tabler/icons-react';
import { useEffect, useState } from 'react';

export default function MapContainer() {
	const [value, setValue] = useState<'Map' | 'Data' | 'Planner'>('Data');

	const pageRender = (page: 'Map' | 'Data' | 'Planner') => {
		switch (page) {
			case 'Map':
				return <MapPlaceholder />;
			case 'Data':
				return <DataPage />;
			case 'Planner':
				return <ThreePlaceholder />;
			default:
				return 'Invalid Render Page';
		}
	};

	return (
		<div className="flex items-center flex-1 flex-col p-2">
			<SegmentedControl
				value={value}
				onChange={(val) => {
					setValue(val as any);
				}}
				data={[
					{
						value: 'Map',
						label: (
							<Center style={{ gap: 10 }}>
								<IconMap style={{ width: rem(16) }} />
							</Center>
						),
					},
					{
						value: 'Data',
						label: (
							<Center style={{ gap: 10 }}>
								<IconTimeline style={{ width: rem(16) }} />
							</Center>
						),
					},
					{
						value: 'Planner',
						label: (
							<Center style={{ gap: 10 }}>
								<IconHexagonalPyramid style={{ width: rem(16) }} />
							</Center>
						),
					},
				]}
			/>

			<div className="flex-1 w-full">{pageRender(value)}</div>
		</div>
	);
}

function MapPlaceholder() {
	return (
		<div className="flex h-full w-full justify-center items-center">
			<IconMap color="#339af0" size={48} />
		</div>
	);
}

function DataPage() {
	const [pose_x] = useVictoryValue('pose/ned/position/x');
	const [pose_y] = useVictoryValue('pose/ned/position/y');

	const [X, setX] = useState(0.0);
	const [Y, setY] = useState(0.0);
	const [extents, setExtents] = useState(60.0);

	useEffect(() => {
		setX((pose_x as number) ?? 0.0);
		setY((pose_y as number) ?? 0.0);
		if (pose_x && pose_y) {
			const max_pos = Math.max(pose_x as number, pose_y as number);
			if (max_pos > extents) {
				setExtents(max_pos + max_pos * 0.1);
			}
		}
	}, [pose_x, pose_y, extents]);

	return (
		<div className="flex h-full w-full justify-center items-center p-2">
			<div className="w-[50%]">
				<ScatterChart
					h={350}
					data={[
						{
							color: 'red.5',
							name: 'X/Y',
							data: [{ x: X, y: Y }],
						}
					]}
					dataKey={{ x: 'x', y: 'y' }}
					xAxisLabel="X Position"
					yAxisLabel="Y Position"
					yAxisProps={{ domain: [-extents, extents] }}
					xAxisProps={{ domain: [-extents, extents] }}
					referenceLines={[
						{ x: 0, label: 'Origin', color: "green.7" },
						{ y: 0, label: 'Origin', color: "blue.7" },
					]}
				/>
			</div>
		</div>
	);
}

function ThreePlaceholder() {
	return (
		<div className="flex h-full w-full justify-center items-center">
			<IconHexagonalPyramid color="#339af0" size={48} />
		</div>
	);
}
