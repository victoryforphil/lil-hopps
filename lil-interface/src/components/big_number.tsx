import useVictoryValue, { MapValue } from "@/hooks/useVictoryValue";
import { LOS } from "./sidebar";

export default function BigNumber(props: { val: number; name?: string; victory_id: string }) {
	const [val] = useVictoryValue(props.victory_id);

    const get_val = (map_val: MapValue | undefined) => {
        if (map_val) {
            return (val as number).toFixed(2)
        } else {
            return (<LOS />)
        }
    }

	return (
		<div className="flex flex-col items-center justify-between bg-[#1f1f1f] p-4 rounded-md w-28 flex-1">
			<div className="text-green-500 text-xs font-mono w-fit ml-auto mb-2">+16</div>
			<div className="font-mono text-3xl">{get_val(val)}</div>
			<div className="opacity-60 text-sm font-extralight">{props.name ?? props.victory_id}</div>
		</div>
	);
}
