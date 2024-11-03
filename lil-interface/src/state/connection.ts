import { create } from 'zustand';

interface ConnectionStore {
    connected: boolean;
    setConnected: (connected: boolean) => void;
}

export const useConnectionStore = create<ConnectionStore>((set) => ({
    connected: true, // Set to false
    setConnected: (connected: boolean) => set({ connected: connected }),
}));
