import useVictoryValue, { MapValue } from "@/hooks/useVictoryValue";
import { LOS } from "./sidebar";
import { useEffect, useState } from "react";

export default function BigNumber(props: { name?: string; victory_id: string }) {
    const [val] = useVictoryValue(props.victory_id);
    const [prevVal, setPrevVal] = useState<number | null>(null);
    const [diff, setDiff] = useState<number>(0);

    useEffect(() => {
        if (val !== undefined && val !== null) {
            if (prevVal !== null) {
                setDiff(val as number - prevVal);
            }
            setPrevVal(val as number);
        }
    }, [val]);

    const get_val = (map_val: MapValue | undefined) => {
        if (map_val) {
            return (val as number).toFixed(2);
        } else {
            return <LOS />;
        }
    };

    // TODO: Why is this re-rendering so much -- IT REALLY SHOULDN'T BE.

    return (
        <div className="flex flex-col items-center justify-between bg-[#1f1f1f] p-4 rounded-md w-28 flex-1">
            <div className={`text-xs font-mono w-fit ml-auto mb-2 ${diff >= 0 ? "text-green-500" : "text-red-500"}`}>
                {diff >= 0 ? `+${diff.toFixed(2)}` : diff.toFixed(2)}
            </div>
            <div className="font-mono text-3xl">{get_val(val)}</div>
            <div className="opacity-60 text-sm font-extralight">{props.name ?? props.victory_id}</div>
        </div>
    );
}