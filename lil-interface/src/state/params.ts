import { create } from 'zustand';
import { subscribeWithSelector } from 'zustand/middleware';

interface ParamStore {
	data: { params: Map<string, string | number | boolean | undefined> };
	setMapValue: (key: string, value: string | number | boolean) => void;
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

const useParamStore = create(
	subscribeWithSelector<ParamStore>((set, get) => ({
		data: { params: new Map() },

		setMapValue: (key, value) => {
			const newData = { params: new Map(get().data.params) };
			newData.params.set(key, value);
			set({ data: newData });
		},

		// hopefully clones map and ignores the things we don't care about.
		overrideMap: (newMap) => {
			set((state) => {
				const updatedParams = new Map(state.data.params);
				newMap.forEach((value, key) => {
					updatedParams.set(key, value);
				});
				return { data: { params: updatedParams } };
			});
		},

		reset: () => {
			set((state) => {
				const new_map = copyMapWithUndefinedValues(state.data.params);
				return { data: { params: new_map } };
			});
		},
	}))
);

export default useParamStore;
