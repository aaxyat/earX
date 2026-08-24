import { useEffect, useState } from "react";
import { Laptop, Power, Minimize2 } from "lucide-react";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";
import { TopHeader } from "@/components/layout/TopHeader";
import { useDeviceStore } from "@/store/useDeviceStore";

export const SystemSettingsPage = () => {
  const { setActiveTab } = useDeviceStore();
  const [autostartEnabled, setAutostartEnabled] = useState(false);
  const [minimizeToTray, setMinimizeToTray] = useState(true);

  useEffect(() => {
    const checkAutostart = async () => {
      try {
        const enabled = await isEnabled();
        setAutostartEnabled(enabled);
      } catch {
        setAutostartEnabled(false);
      }
    };
    checkAutostart();
  }, []);

  const handleToggleAutostart = async () => {
    try {
      if (autostartEnabled) {
        await disable();
        setAutostartEnabled(false);
      } else {
        await enable();
        setAutostartEnabled(true);
      }
    } catch (e) {
      console.warn("Failed to toggle autostart", e);
      setAutostartEnabled(!autostartEnabled);
    }
  };

  return (
    <div className="flex flex-col min-h-screen bg-black text-white pb-6 animate-fadeIn">
      {/* Top Header */}
      <TopHeader
        title="System settings"
        showBack={true}
        onBack={() => setActiveTab("dashboard")}
        showEdit={false}
      />

      <div className="flex-1 overflow-y-auto px-4 pt-3 flex flex-col gap-3">
        {/* Launch at System Startup */}
        <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Power className="w-5 h-5 text-white/80" />
            <div>
              <h4 className="text-base font-medium text-white">
                Launch at startup
              </h4>
              <p className="text-xs text-nothing-grey mt-0.5">
                Automatically start earX in tray on boot
              </p>
            </div>
          </div>
          <button
            onClick={handleToggleAutostart}
            className={`w-12 h-7 flex items-center rounded-full p-1 transition-colors duration-200 ${
              autostartEnabled ? "bg-white" : "bg-zinc-800"
            }`}
            aria-label="Toggle Launch at startup"
          >
            <div
              className={`bg-black w-5 h-5 rounded-full shadow-md transform transition-transform duration-200 ${
                autostartEnabled ? "translate-x-5" : "translate-x-0 bg-white"
              }`}
            />
          </button>
        </div>

        {/* Minimize to Tray on Close */}
        <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between">
          <div className="flex items-center gap-3">
            <Minimize2 className="w-5 h-5 text-white/80" />
            <div>
              <h4 className="text-base font-medium text-white">
                Minimize to tray
              </h4>
              <p className="text-xs text-nothing-grey mt-0.5">
                Keep running in background when closed
              </p>
            </div>
          </div>
          <button
            onClick={() => setMinimizeToTray(!minimizeToTray)}
            className={`w-12 h-7 flex items-center rounded-full p-1 transition-colors duration-200 ${
              minimizeToTray ? "bg-white" : "bg-zinc-800"
            }`}
            aria-label="Toggle Minimize to tray"
          >
            <div
              className={`bg-black w-5 h-5 rounded-full shadow-md transform transition-transform duration-200 ${
                minimizeToTray ? "translate-x-5" : "translate-x-0 bg-white"
              }`}
            />
          </button>
        </div>

        {/* About earX card */}
        <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex flex-col gap-3 mt-3">
          <div className="flex items-center gap-3 mb-1">
            <Laptop className="w-5 h-5 text-nothing-red" />
            <h4 className="text-base font-medium text-white">About earX</h4>
          </div>
          <p className="text-xs text-nothing-grey leading-relaxed">
            earX is a native, ultra-lightweight desktop controller for Nothing & CMF audio devices, built with Tauri v2 and Rust.
          </p>
          <div className="flex justify-between items-center text-xs pt-3 border-t border-white/5 text-zinc-400">
            <span>Version</span>
            <span className="font-mono text-white">0.0.1-alpha</span>
          </div>
          <div className="flex justify-between items-center text-xs text-zinc-400">
            <span>Platform</span>
            <span className="font-mono text-white">Windows & macOS</span>
          </div>
        </div>
      </div>
    </div>
  );
};
