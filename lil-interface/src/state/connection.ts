import { create } from 'zustand';

interface ConnectionStore {
    connected: boolean;
    connecting: boolean;
    setConnected: (connected: boolean) => void;
    setConnecting: (connecting: boolean) => void;
}

export const useConnectionStore = create<ConnectionStore>((set) => ({
    connected: true, // TODO: Set to false for prod
    connecting: false,
    setConnected: (connected: boolean) => set({ connected: connected, connecting: false }),
    setConnecting: (connecting: boolean) => set({ connecting: connecting }),
}));
