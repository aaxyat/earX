import {
  Hand,
  Layers,
  Settings,
  Info,
  Laptop,
} from "lucide-react";
import { ActiveTab, useDeviceStore } from "@/store/useDeviceStore";

interface BentoCardGridProps {
  onNavigate: (tab: ActiveTab) => void;
}

export const BentoCardGrid = ({ onNavigate }: BentoCardGridProps) => {
  const {
    device,
    setUltraBass,
    setLowLatency,
    setDualConnection,
  } = useDeviceStore();

  const { eq, low_latency_mode, dual_connection, device_name } = device;

  const handleUltraBassToggle = () => {
    setUltraBass(!eq.ultra_bass_enabled, eq.ultra_bass_level || 2);
  };

  const handleUltraBassLevelCycle = () => {
    const nextLevel = (eq.ultra_bass_level % 5) + 1;
    setUltraBass(true, nextLevel);
  };

  return (
    <div className="grid grid-cols-2 gap-3 px-4 my-2 mb-8 w-full">
      {/* 1. Ultra Bass Card */}
      <div className="bg-[#1c1c1e] rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md">
        <div>
          <h4 className="text-base font-medium text-white">Ultra bass</h4>
          <p className="text-xs text-nothing-grey mt-0.5">
            {eq.ultra_bass_enabled ? `On · Level ${eq.ultra_bass_level}` : "Off"}
          </p>
        </div>

        <div className="flex items-center justify-between mt-auto pt-2">
          {/* Switch Toggle */}
          <button
            onClick={handleUltraBassToggle}
            className={`w-12 h-7 flex items-center rounded-full p-1 transition-colors duration-200 ${
              eq.ultra_bass_enabled ? "bg-white" : "bg-zinc-800"
            }`}
            aria-label="Toggle Ultra Bass"
          >
            <div
              className={`bg-black w-5 h-5 rounded-full shadow-md transform transition-transform duration-200 ${
                eq.ultra_bass_enabled ? "translate-x-5" : "translate-x-0 bg-white"
              }`}
            />
          </button>

          {/* 5-Bar Red Level Meter */}
          <button
            onClick={handleUltraBassLevelCycle}
            className="flex items-center gap-1 p-1 cursor-pointer hover:opacity-80 transition-opacity"
            title="Click to cycle Ultra Bass level (1-5)"
          >
            {[1, 2, 3, 4, 5].map((bar) => {
              const isActive = eq.ultra_bass_enabled && bar <= eq.ultra_bass_level;
              return (
                <div
                  key={bar}
                  className={`w-1 h-5 rounded-full transition-all duration-200 ${
                    isActive ? "bg-nothing-red shadow-[0_0_6px_#d71920]" : "bg-zinc-800"
                  }`}
                />
              );
            })}
          </button>
        </div>
      </div>

      {/* 2. Personal Sound Profile Card */}
      <div className="bg-[#1c1c1e] rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md">
        <div>
          <h4 className="text-base font-medium text-white">
            Personal Sound Profile
          </h4>
          <p className="text-xs text-nothing-grey mt-0.5">Off</p>
        </div>

        <div className="mt-auto self-start">
          {/* Dot Matrix Graphic Icon */}
          <div className="grid grid-cols-3 gap-1 p-1">
            <div className="w-1.5 h-1.5 rounded-full bg-zinc-500" />
            <div className="w-1.5 h-1.5 rounded-full bg-zinc-300" />
            <div className="w-1.5 h-1.5 rounded-full bg-zinc-500" />
            <div className="w-1.5 h-1.5 rounded-full bg-zinc-300" />
            <div className="w-1.5 h-1.5 rounded-full bg-white" />
            <div className="w-1.5 h-1.5 rounded-full bg-zinc-300" />
            <div className="w-1.5 h-1.5 rounded-full bg-zinc-500" />
            <div className="w-1.5 h-1.5 rounded-full bg-zinc-300" />
            <div className="w-1.5 h-1.5 rounded-full bg-zinc-500" />
          </div>
        </div>
      </div>

      {/* 3. Equalizer Card */}
      <button
        onClick={() => onNavigate("equalizer")}
        className="bg-[#1c1c1e] hover:bg-[#262628] active:scale-[0.98] transition-all rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md text-left group"
      >
        <div>
          <h4 className="text-base font-medium text-white">Equalizer</h4>
          <p className="text-xs text-nothing-grey mt-0.5">{eq.preset || "Rock"}</p>
        </div>

        <div className="mt-auto">
          {/* Equalizer multi-sliders icon */}
          <div className="flex items-end gap-1 text-white/80 group-hover:text-white transition-colors">
            <div className="w-1 h-3 bg-white rounded-full" />
            <div className="w-1 h-6 bg-white rounded-full" />
            <div className="w-1 h-4 bg-white rounded-full" />
            <div className="w-1 h-2 bg-white rounded-full" />
          </div>
        </div>
      </button>

      {/* 4. Controls Card */}
      <button
        onClick={() => onNavigate("gestures")}
        className="bg-[#1c1c1e] hover:bg-[#262628] active:scale-[0.98] transition-all rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md text-left group"
      >
        <div>
          <h4 className="text-base font-medium text-white">Controls</h4>
          <p className="text-xs text-nothing-grey mt-0.5">Customised</p>
        </div>

        <div className="mt-auto">
          <Hand className="w-6 h-6 text-white/80 group-hover:text-white transition-colors" />
        </div>
      </button>

      {/* 5. Low Lag Mode Card */}
      <div className="bg-[#1c1c1e] rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md">
        <div>
          <h4 className="text-base font-medium text-white">Low lag mode</h4>
          <p className="text-xs text-nothing-grey mt-0.5">
            {low_latency_mode ? "On" : "Off"}
          </p>
        </div>

        <div className="mt-auto">
          <button
            onClick={() => setLowLatency(!low_latency_mode)}
            className={`w-12 h-7 flex items-center rounded-full p-1 transition-colors duration-200 ${
              low_latency_mode ? "bg-white" : "bg-zinc-800"
            }`}
            aria-label="Toggle Low Lag Mode"
          >
            <div
              className={`bg-black w-5 h-5 rounded-full shadow-md transform transition-transform duration-200 ${
                low_latency_mode ? "translate-x-5" : "translate-x-0 bg-white"
              }`}
            />
          </button>
        </div>
      </div>

      {/* 6. Dual Connection Card */}
      <div className="bg-[#1c1c1e] rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md">
        <div>
          <h4 className="text-base font-medium text-white">Dual connection</h4>
          <p className="text-xs text-nothing-grey mt-0.5">
            {dual_connection ? "On" : "Off"}
          </p>
        </div>

        <div className="mt-auto">
          <button
            onClick={() => setDualConnection(!dual_connection)}
            className="p-1 -ml-1 text-white/80 hover:text-white"
            aria-label="Dual Connection"
          >
            <Layers className="w-6 h-6" />
          </button>
        </div>
      </div>

      {/* 7. Device Settings Card */}
      <button
        onClick={() => onNavigate("device_settings")}
        className="bg-[#1c1c1e] hover:bg-[#262628] active:scale-[0.98] transition-all rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md text-left group"
      >
        <div>
          <h4 className="text-base font-medium text-white">Device settings</h4>
        </div>

        <div className="mt-auto">
          <Settings className="w-6 h-6 text-white/80 group-hover:text-white transition-colors" />
        </div>
      </button>

      {/* 8. About Card */}
      <div className="bg-[#1c1c1e] rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md">
        <div>
          <h4 className="text-base font-medium text-white">About</h4>
          <p className="text-xs text-nothing-grey mt-0.5 truncate">{device_name}</p>
        </div>

        <div className="mt-auto">
          <Info className="w-6 h-6 text-white/80" />
        </div>
      </div>

      {/* 9. System Settings Card */}
      <button
        onClick={() => onNavigate("system_settings")}
        className="bg-[#1c1c1e] hover:bg-[#262628] active:scale-[0.98] transition-all rounded-3xl p-4 flex flex-col justify-between h-40 border border-white/5 shadow-md text-left group"
      >
        <div>
          <h4 className="text-base font-medium text-white">System settings</h4>
          <p className="text-xs text-nothing-grey mt-0.5">Autostart & Tray</p>
        </div>

        <div className="mt-auto">
          <Laptop className="w-6 h-6 text-white/80 group-hover:text-white transition-colors" />
        </div>
      </button>
    </div>
  );
};
