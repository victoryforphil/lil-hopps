import { create } from 'zustand';
import { subscribeWithSelector } from 'zustand/middleware'

interface ControlStore {
	armed: boolean;
    flying: boolean;
	arm: () => void;
	disarm: () => void;
    toggleArm: () => void;
    toggleFlying: () => void;
}

const useControlStore = create<ControlStore>()(
    subscribeWithSelector((set) => ({
        armed: false,
        flying: false,
        arm: () => set({ armed: true }),
        disarm: () => set({ armed: false }),
        toggleArm: () => set((state) => {return { armed: !state.armed }}),
        toggleFlying: () => set((state) => {return { flying: !state.flying }}),
    })),
  )

// Handlers for data changing -- send message to ws.

// useControlStore.subscribe((state) => state.flying, (flying) => {
//     console.log("FLYING CHANGED -- SEND MESSAGE")
// })

// useControlStore.subscribe((state) => state.armed, (armed) => {
//     console.log("ARMED CHANGED -- SEND MESSAGE")
// })

export default useControlStore;
