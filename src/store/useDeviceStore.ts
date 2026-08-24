import { create } from "zustand";
import { AncMode, DeviceState } from "@/types/device";
import {
  connectDevice,
  disconnectDevice,
  getDeviceState,
  listenToDeviceState,
  ringEarbuds,
  setAncMode,
  setCustomEq,
  setInEarDetection,
  setLowLatencyMode,
  setUltraBass,
} from "@/lib/tauri";

export type ActiveTab =
  | "dashboard"
  | "equalizer"
  | "gestures"
  | "device_settings"
  | "system_settings";

interface DeviceStore {
  device: DeviceState;
  isLoading: boolean;
  activeTab: ActiveTab;
  isInitialized: boolean;

  // Actions
  init: () => Promise<void>;
  connect: (address: string, name?: string) => Promise<void>;
  disconnect: () => Promise<void>;
  setAnc: (mode: AncMode) => Promise<void>;
  setUltraBass: (enabled: boolean, level: number) => Promise<void>;
  setCustomEq: (bass: number, mid: number, treble: number) => Promise<void>;
  setInEar: (enabled: boolean) => Promise<void>;
  setLowLatency: (enabled: boolean) => Promise<void>;
  setSpatialAudio: (enabled: boolean) => void;
  setDualConnection: (enabled: boolean) => void;
  ringBuds: (isLeft: boolean, start: boolean) => Promise<void>;
  setActiveTab: (tab: ActiveTab) => void;
  setDeviceName: (name: string) => void;
}

const initialDeviceState: DeviceState = {
  is_connected: true,
  device_name: "Ayush's CMF Buds 2 Plus",
  address: "AA:BB:CC:DD:EE:FF",
  serial_number: "SH247900123456",
  firmware_version: "1.0.1.37",
  model: {
    name: "CMF Buds Pro 2",
    base: "B172",
    code_name: "espeon",
    color_variant: "blue",
    is_anc_supported: true,
    is_ultra_bass_supported: true,
    is_advanced_eq_supported: false,
    is_listening_mode_device: true,
    is_fit_test_supported: true,
    is_case_led_supported: false,
    left_image: "/assets/espeon_blue_left.webp",
    right_image: "/assets/espeon_blue_right.webp",
    case_image: "/assets/espeon_blue_case.webp",
    duo_image: "",
  },
  battery: {
    left: 95,
    right: 90,
    case: 40,
    is_charging_left: false,
    is_charging_right: false,
    is_charging_case: true,
  },
  anc_mode: {
    type: "NoiseCancellation",
    level: "High",
  },
  eq: {
    preset: "Rock",
    custom_bass: 0,
    custom_mid: 0,
    custom_treble: 0,
    ultra_bass_enabled: true,
    ultra_bass_level: 2,
  },
  in_ear_detection: true,
  low_latency_mode: false,
  spatial_audio: false,
  dual_connection: true,
};

export const useDeviceStore = create<DeviceStore>((set, get) => ({
  device: initialDeviceState,
  isLoading: false,
  activeTab: "dashboard",
  isInitialized: false,

  init: async () => {
    if (get().isInitialized) return;
    try {
      // Connect real-time state change listener
      await listenToDeviceState((state) => {
        set({ device: state });
      });

      // Try initial state fetch
      const state = await getDeviceState();
      if (state && state.is_connected) {
        set({ device: state, isInitialized: true });
      } else {
        // Run mock connect if not connected
        await get().connect("AA:BB:CC:DD:EE:FF", "Ayush's CMF Buds 2 Plus");
        set({ isInitialized: true });
      }
    } catch {
      // Running in browser dev mode without Tauri backend
      set({ isInitialized: true });
    }
  },

  connect: async (address: string, name?: string) => {
    set({ isLoading: true });
    try {
      const state = await connectDevice(address, name);
      set({ device: state, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },

  disconnect: async () => {
    set({ isLoading: true });
    try {
      await disconnectDevice();
      set((prev) => ({
        device: { ...prev.device, is_connected: false },
        isLoading: false,
      }));
    } catch {
      set({ isLoading: false });
    }
  },

  setAnc: async (mode: AncMode) => {
    // Optimistic update
    set((prev) => ({
      device: { ...prev.device, anc_mode: mode },
    }));
    try {
      await setAncMode(mode);
    } catch (e) {
      console.warn("Failed to set ANC mode on device", e);
    }
  },

  setUltraBass: async (enabled: boolean, level: number) => {
    // Optimistic update
    set((prev) => ({
      device: {
        ...prev.device,
        eq: {
          ...prev.device.eq,
          ultra_bass_enabled: enabled,
          ultra_bass_level: level,
        },
      },
    }));
    try {
      await setUltraBass(enabled, level);
    } catch (e) {
      console.warn("Failed to set Ultra Bass", e);
    }
  },

  setCustomEq: async (bass: number, mid: number, treble: number) => {
    set((prev) => ({
      device: {
        ...prev.device,
        eq: {
          ...prev.device.eq,
          custom_bass: bass,
          custom_mid: mid,
          custom_treble: treble,
          preset: "Custom",
        },
      },
    }));
    try {
      await setCustomEq(bass, mid, treble);
    } catch (e) {
      console.warn("Failed to set Custom EQ", e);
    }
  },

  setInEar: async (enabled: boolean) => {
    set((prev) => ({
      device: { ...prev.device, in_ear_detection: enabled },
    }));
    try {
      await setInEarDetection(enabled);
    } catch (e) {
      console.warn("Failed to set In-Ear detection", e);
    }
  },

  setLowLatency: async (enabled: boolean) => {
    set((prev) => ({
      device: { ...prev.device, low_latency_mode: enabled },
    }));
    try {
      await setLowLatencyMode(enabled);
    } catch (e) {
      console.warn("Failed to set Low Latency mode", e);
    }
  },

  setSpatialAudio: (enabled: boolean) => {
    set((prev) => ({
      device: { ...prev.device, spatial_audio: enabled },
    }));
  },

  setDualConnection: (enabled: boolean) => {
    set((prev) => ({
      device: { ...prev.device, dual_connection: enabled },
    }));
  },

  ringBuds: async (isLeft: boolean, start: boolean) => {
    try {
      await ringEarbuds(isLeft, start);
    } catch (e) {
      console.warn("Failed to ring earbuds", e);
    }
  },

  setActiveTab: (tab: ActiveTab) => set({ activeTab: tab }),

  setDeviceName: (name: string) =>
    set((prev) => ({ device: { ...prev.device, device_name: name } })),
}));
