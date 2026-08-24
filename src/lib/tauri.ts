import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { AncMode, DeviceState, DiscoveredDevice } from "@/types/device";

export const getDeviceState = async (): Promise<DeviceState> => {
  return await invoke<DeviceState>("get_device_state");
};

export const scanDevices = async (): Promise<DiscoveredDevice[]> => {
  return await invoke<DiscoveredDevice[]>("scan_devices");
};

export const connectDevice = async (
  address: string,
  name?: string
): Promise<DeviceState> => {
  return await invoke<DeviceState>("connect_device", { address, name });
};

export const disconnectDevice = async (): Promise<void> => {
  return await invoke<void>("disconnect_device");
};

export const setAncMode = async (mode: AncMode): Promise<DeviceState> => {
  return await invoke<DeviceState>("set_anc", { mode });
};

export const setUltraBass = async (
  enabled: boolean,
  level: number
): Promise<DeviceState> => {
  return await invoke<DeviceState>("set_ultra_bass", { enabled, level });
};

export const setCustomEq = async (
  bass: number,
  mid: number,
  treble: number
): Promise<DeviceState> => {
  return await invoke<DeviceState>("set_custom_eq", { bass, mid, treble });
};

export const setInEarDetection = async (
  enabled: boolean
): Promise<DeviceState> => {
  return await invoke<DeviceState>("set_in_ear", { enabled });
};

export const setLowLatencyMode = async (
  enabled: boolean
): Promise<DeviceState> => {
  return await invoke<DeviceState>("set_low_latency", { enabled });
};

export const ringEarbuds = async (
  isLeft: boolean,
  startRing: boolean
): Promise<void> => {
  return await invoke<void>("ring_earbuds", { isLeft, startRing });
};

export const listenToDeviceState = async (
  callback: (state: DeviceState) => void
): Promise<UnlistenFn> => {
  return await listen<DeviceState>("device-state-changed", (event) => {
    callback(event.payload);
  });
};
