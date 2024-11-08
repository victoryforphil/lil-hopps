import { create } from 'zustand';
import { subscribeWithSelector } from 'zustand/middleware';

interface DroneStore {
	data: Map<string, string | number | boolean | undefined>;
	setMapValue: (key: string, value: string | number | boolean) => void;
	getMapValue: (key: string) => string | number | boolean | undefined;
	overrideMap: (newMap: Map<string, string | number | boolean>) => void;
	reset: () => void;
}

function copyMapWithUndefinedValues<K, V>(map: Map<K, V>): Map<K, V | undefined> {
    const newMap = new Map<K, V | undefined>();
    map.forEach((_, key) => {
        newMap.set(key, undefined);
    });
    return newMap;
}

const useDroneStore = create(
	subscribeWithSelector<DroneStore>((set, get) => ({
		data: new Map(),

		setMapValue: (key, value) => {
			const newData = new Map(get().data);
			newData.set(key, value);
			set({ data: newData });
		},

		getMapValue: (key) => get().data.get(key),

        // hopefully clones map and ignores the things we don't care about.
		overrideMap: (newMap) => {
			set({ data: newMap });
		},
		reset: () => {
			set((state) => {
				let new_map = copyMapWithUndefinedValues(state.data);
				return { data: new_map }
			})
		}
	}))
);

export default useDroneStore;
