import { create } from 'zustand';

interface LogStore {
	log_messages: string[];
	addLogMessage: (message: string) => void;
	reset: () => void;
}

export const useLogStore = create<LogStore>((set) => ({
	log_messages: [],
	// This is wrong somehow. TBD
	addLogMessage: (message: string) =>
		set((state) => {
			return { log_messages: [...state.log_messages, message] };
		}),
	reset: () => {
		set({ log_messages: [] });
	},
}));
