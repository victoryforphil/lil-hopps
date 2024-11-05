import { create } from 'zustand';

interface ConnectionStore {
    connected: boolean;
    connecting: boolean;
    lastRecieved: number;
    setConnected: (connected: boolean) => void;
    setConnecting: (connecting: boolean) => void;
    setRecieved: (timestamp_ms: number) => void;
}

export const useConnectionStore = create<ConnectionStore>((set) => ({
    connected: true,
    connecting: false,
    lastRecieved: 0,
    setConnected: (connected: boolean) => set({ connected: connected, connecting: false }),
    setConnecting: (connecting: boolean) => set({ connecting: connecting }),
    setRecieved: (timestamp_ms: number) => set({ lastRecieved: timestamp_ms })
}));
