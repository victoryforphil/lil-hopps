import useVictoryValue from '@/hooks/useVictoryValue';
import useParamStore from '@/state/params';
import { ScatterChart } from '@mantine/charts';
import { Autocomplete, Center, rem, ScrollArea, SegmentedControl } from '@mantine/core';
import { IconHexagonalPyramid, IconMap, IconSearch, IconTimeline, IconVariable } from '@tabler/icons-react';
import { useCallback, useEffect, useState } from 'react';

export default function LargeContentView() {
	const [value, setValue] = useState<'Map' | 'Data' | 'Planner' | 'Params'>('Data');

	const pageRender = (page: 'Map' | 'Data' | 'Planner' | 'Params') => {
		switch (page) {
			case 'Map':
				return <MapPlaceholder />;
			case 'Data':
				return <DataPage />;
			case 'Planner':
				return <ThreePlaceholder />;
			case 'Params':
				return <ParamView />;
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
					{
						value: 'Params',
						label: (
							<Center style={{ gap: 10 }}>
								<IconVariable style={{ width: rem(16) }} />
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
						},
					]}
					dataKey={{ x: 'x', y: 'y' }}
					xAxisLabel="X Position"
					yAxisLabel="Y Position"
					yAxisProps={{ domain: [-extents, extents] }}
					xAxisProps={{ domain: [-extents, extents] }}
					referenceLines={[
						{ x: 0, label: 'Origin', color: 'green.7' },
						{ y: 0, label: 'Origin', color: 'blue.7' },
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

function ParamView() {
	const [filter, setFilter] = useState('');

	const paramMap = useParamStore((state) => state.data);


	const generateLineItems = useCallback(
	  (params: Map<string, number | string | boolean | undefined>) => {
		let items = [];

		for (const param of params.entries()) {
			if (filter === "" || param[0].includes(filter)) {
				items.push(
					<div key={param[0]} className="flex justify-between">
						<div className="text-lg font-mono">{param[0]}</div>
						<div>{param[1]}</div>
					</div>
				);
			}
		}

		return items;
	  },
	  [filter],
	)

	return (
		<div className="flex w-full justify-center items-center p-4 flex-1 h-full flex-col">
			<Autocomplete
				placeholder="Search for param"
				data={Array.from(paramMap.params.keys())}
				className="w-80"
				limit={10}
				leftSectionPointerEvents="none"
				leftSection={<IconSearch style={{ width: rem(16), height: rem(16) }} />}
				value={filter}
				onChange={setFilter}
			/>
			<ScrollArea className="w-full h-[75svh] flex gap-5 p-5">{generateLineItems(paramMap.params)}</ScrollArea>
		</div>
	);
}
