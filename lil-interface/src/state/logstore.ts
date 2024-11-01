import { create } from 'zustand';

interface LogStore {
    log_message: string;
    setLogMessage: (message: string) => void;
}

export const useLogStore = create<LogStore>((set) => ({
    log_message: "",
    setLogMessage: (message: string) => set({ log_message: message }),
}));