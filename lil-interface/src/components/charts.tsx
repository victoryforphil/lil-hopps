import useDroneStore from '@/state/drone';
import { LineChart } from '@mantine/charts';
import { useEffect, useState } from 'react';

function roundFloat(value: number, decimalPlaces: number): number {
	const multiplier = Math.pow(10, decimalPlaces);
	return Math.round(value * multiplier) / multiplier;
}

export default function RealtimeLine(props: { element_limit: number; title: string; victory_id: string }) {
	// FULL MAP -- NO REDRAW. KTHX BIY
	const drone_store = useDroneStore.getState().data;

	const [points, setPoints] = useState<Array<{ z: number; x: number }>>([]);

	useEffect(() => {
		const intervalId = setInterval(() => {
			const drone_val = drone_store.get(props.victory_id);

			if (drone_val) {
				setPoints((prevPoints) => {
					const now = Date.now();
					const updatedPoints = [...prevPoints, { z: roundFloat(drone_val as number, 3), x: now }];

					if (updatedPoints.length > props.element_limit) {
						updatedPoints.shift();
					}

					return updatedPoints;
				});
			}
			// For testing.
			// } else {
			// 	const randomZ = Math.random() * 10;
			// 	setPoints((prevPoints) => {
			// 		const now = Date.now();
			// 		const updatedPoints = [...prevPoints, { z: randomZ, x: now }];

			// 		if (updatedPoints.length > props.element_limit) {
			// 			updatedPoints.shift();
			// 		}

			// 		return updatedPoints;
			// 	});
			// }
		}, 100);

		return () => {
			clearInterval(intervalId);
		};
	}, [drone_store, props.element_limit, props.victory_id]);

	return (
		<div className="w-[48%] bg-[#1f1f1f] shadow-sm rounded-md">
			<div className="w-fit p-4 font-mono">{props.victory_id}</div>
			<hr className="h-px mb-2 bg-gray-200 border-0 dark:bg-zinc-700"></hr>
			<div>
				<LineChart
					h={300}
					data={points}
					// xAxisLabel="Time"
					// yAxisLabel="Z Position"
					series={[{ name: 'z', color: 'indigo.6' }]}
					curveType="linear"
					dataKey={''}
					referenceLines={[{ y: 5, label: 'Nominal', color: 'green.7' }]}
					// yAxisProps={{ domain: [0, 10.0] }}
					// xAxisProps={{ domain: [-extents, extents] }}

					withXAxis={false}
					withYAxis={false}
					withDots={false}
					gridAxis="none"
				/>
			</div>
		</div>
	);
}
