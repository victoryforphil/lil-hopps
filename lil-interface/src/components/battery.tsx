import clsx from "clsx";

export function Battery(props: { charge: number }) {
    const breakpoints = [0, 10, 40];

    if (props.charge > breakpoints[2]) {
        return (
            <div className="battery">
                <div className={clsx("battery-level")} style={{ width: props.charge + "%" }}></div>
            </div>
        );
    } else if (props.charge > breakpoints[1]) {
        return (
            <div className="battery">
                <div className={clsx("battery-level", "warn")} style={{ width: props.charge + "%" }}></div>
            </div>
        );
    } else if (props.charge > breakpoints[0]) {
        return (
            <div className="battery">
                <div className={clsx("battery-level", "alert")} style={{ width: props.charge + "%" }}></div>
            </div>
        );
    }

    return (
        <div className="battery">
            <div className={clsx("battery-level", "warn")} style={{ width: props.charge + "%" }}></div>
        </div>
    );
}

export function BatteryLabel(props: { charge: number}) {
    return (
        <div className="flex gap-1 items-center">
            <div className="text-xs">
                {props.charge}%
            </div>
            <Battery charge={props.charge} />
        </div>
    )
}
