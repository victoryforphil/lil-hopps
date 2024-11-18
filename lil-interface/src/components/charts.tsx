import useVictoryValue from '@/hooks/useVictoryValue';
import { LineChart } from '@mantine/charts';
import { useEffect, useState } from 'react';

export default function RealtimeLine(props: { element_limit: number }) {
	const [pose_z] = useVictoryValue('pose/ned/position/z');

	const [points, setPoints] = useState<Array<{ z: number; timestamp: number }>>([]);

	useEffect(() => {
		const intervalId = setInterval(() => {
			if (pose_z !== undefined) {
				setPoints((prevPoints) => {
					const now = Date.now();
					const updatedPoints = [...prevPoints, { z: pose_z as number, timestamp: now }];

					if (updatedPoints.length > props.element_limit) {
						updatedPoints.shift();
					}

					return updatedPoints;
				});
			} else {
				const randomZ = Math.random() * 10;
				setPoints((prevPoints) => {
					const now = Date.now();
					const updatedPoints = [...prevPoints, { z: randomZ, timestamp: now }];

					if (updatedPoints.length > props.element_limit) {
						updatedPoints.shift();
					}

					return updatedPoints;
				});
			}
		}, 100); // 0.5 seconds interval

		return () => {
			clearInterval(intervalId);
		};
	}, [pose_z, props.element_limit]);

	return (
		<div className="w-[48%] bg-[#1f1f1f] shadow-sm rounded-md">
			<div className="w-fit p-4">Z index</div>
			<hr className="h-px mb-2 bg-gray-200 border-0 dark:bg-zinc-700"></hr>
			<div>
				<LineChart
					h={300}
					data={points.map((point) => ({ x: point.timestamp, z: point.z }))}
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
