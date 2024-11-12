import { useGCSConnection } from '@/data/ws.singleton';
import useVictoryValue from '@/hooks/useVictoryValue';
import useParamStore from '@/state/params';
import { ScatterChart } from '@mantine/charts';
import { Autocomplete, Center, NumberInput, rem, ScrollArea, SegmentedControl } from '@mantine/core';
import { useDebouncedValue } from '@mantine/hooks';
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
			const items = [];

			for (const param of params.entries()) {
				if (filter === '' || param[0].includes(filter)) {
					items.push(<ParamField key={param[0]} name={param[0]} value={param[1]} />);
				}
			}

			return items;
		},
		[filter]
	);

	return (
		<div className="flex w-[50%] p-4 h-full flex-col">
			<div className="w-fit text-lg font-bold pb-2">Drone Param Settings</div>
			<Autocomplete
				placeholder="Search for param"
				data={Array.from(paramMap.params.keys())}
				className="w-full"
				limit={5}
				leftSectionPointerEvents="none"
				leftSection={<IconSearch style={{ width: rem(16), height: rem(16) }} />}
				value={filter}
				onChange={setFilter}
				comboboxProps={{ position: 'right', middlewares: { flip: false, shift: true } }}
			/>
			<ScrollArea className="w-full h-[70svh] flex gap-5 m-2 p-2 pr-5 border-y-2 rounded-lg border-zinc-500">
				{generateLineItems(paramMap.params)}
			</ScrollArea>
		</div>
	);
}

function ParamField(props: { name: string; value: string | number | boolean | undefined }) {
	const [value, setValue] = useState<string | number>(props.value as string);

	// After 1 seconds go ahead and set the real value.
	const [debounced] = useDebouncedValue(value, 1000);

	useEffect(() => {
		if (debounced !== props.value) {
			console.log('Sending a value back to GCS');
			useGCSConnection().sendMessage(`PARAM:params/${props.name}:${debounced}`);
		}
	}, [debounced]);

	return (
		<div className="flex justify-between items-center bg-zinc-800 rounded-md p-2 m-2 px-6 font-mono hover:bg-zinc-900">
			<div className="text-lg">{props.name}</div>
			<NumberInput variant="unstyled" value={value} onChange={setValue} />
		</div>
	);
}
