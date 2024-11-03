import { create } from 'zustand';

interface ControlStore {
	armed: boolean;
    flying: boolean;
	arm: () => void;
	disarm: () => void;
    toggleArm: () => void;
    toggleFlying: () => void;
}

const useControlStore = create<ControlStore>((set) => ({
	armed: false,
    flying: false,
	arm: () => set({ armed: true }),
	disarm: () => set({ armed: false }),
    toggleArm: () => set((state) => {return { armed: !state.armed }}),
    toggleFlying: () => set((state) => {return { flying: !state.flying }}),
}));

export default useControlStore;
