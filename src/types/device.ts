export type BaseId =
  | "B181"
  | "B157"
  | "B155"
  | "B163"
  | "B164"
  | "B168"
  | "B171"
  | "B162"
  | "B172"
  | "B174"
  | "UNKNOWN";

export interface DeviceModelInfo {
  name: string;
  base: BaseId;
  code_name: string;
  color_variant: string;
  is_anc_supported: boolean;
  is_ultra_bass_supported: boolean;
  is_advanced_eq_supported: boolean;
  is_listening_mode_device: boolean;
  is_fit_test_supported: boolean;
  is_case_led_supported: boolean;
  left_image: string;
  right_image: string;
  case_image: string;
  duo_image: string;
}

export interface BatteryTelemetry {
  left: number | null;
  right: number | null;
  case: number | null;
  is_charging_left: boolean;
  is_charging_right: boolean;
  is_charging_case: boolean;
}

export type AncLevel = "Low" | "Mid" | "High" | "Adaptive";

export type AncMode =
  | { type: "Off" }
  | { type: "Transparency" }
  | { type: "NoiseCancellation"; level: AncLevel };

export interface EqSettings {
  preset: string; // "Balanced" | "More Bass" | "More Treble" | "Voice" | "Rock" | "Pop" | "Custom"
  custom_bass: number;
  custom_mid: number;
  custom_treble: number;
  ultra_bass_enabled: boolean;
  ultra_bass_level: number; // 0 .. 5
}

export interface DeviceState {
  is_connected: boolean;
  device_name: string;
  address: string;
  serial_number: string | null;
  firmware_version: string | null;
  model: DeviceModelInfo | null;
  battery: BatteryTelemetry;
  anc_mode: AncMode;
  eq: EqSettings;
  in_ear_detection: boolean;
  low_latency_mode: boolean;
  spatial_audio: boolean;
  dual_connection: boolean;
}

export interface DiscoveredDevice {
  name: string;
  address: string;
  is_connected: boolean;
}
