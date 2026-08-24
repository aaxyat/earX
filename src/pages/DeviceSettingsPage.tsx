import { useState } from "react";
import { ChevronRight, Sparkles, Volume2 } from "lucide-react";
import { TopHeader } from "@/components/layout/TopHeader";
import { EarTipFitTestModal } from "@/components/settings/EarTipFitTestModal";
import { FindMyBudsModal } from "@/components/settings/FindMyBudsModal";
import { useDeviceStore } from "@/store/useDeviceStore";

export const DeviceSettingsPage = () => {
  const { device, setInEar, setLowLatency, setActiveTab } = useDeviceStore();
  const [isFitTestOpen, setIsFitTestOpen] = useState(false);
  const [isFindBudsOpen, setIsFindBudsOpen] = useState(false);

  const { in_ear_detection, low_latency_mode, firmware_version, serial_number } =
    device;

  return (
    <div className="flex flex-col min-h-screen bg-black text-white pb-6 animate-fadeIn">
      {/* Top Header */}
      <TopHeader
        title="Device settings"
        showBack={true}
        onBack={() => setActiveTab("dashboard")}
        showEdit={false}
      />

      <div className="flex-1 overflow-y-auto px-4 pt-3 flex flex-col gap-3">
        {/* In-Ear Detection Toggle */}
        <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between">
          <div>
            <h4 className="text-base font-medium text-white">In-ear detection</h4>
            <p className="text-xs text-nothing-grey mt-0.5">
              Auto-pauses audio when earbud is removed
            </p>
          </div>
          <button
            onClick={() => setInEar(!in_ear_detection)}
            className={`w-12 h-7 flex items-center rounded-full p-1 transition-colors duration-200 ${
              in_ear_detection ? "bg-white" : "bg-zinc-800"
            }`}
            aria-label="Toggle In-Ear Detection"
          >
            <div
              className={`bg-black w-5 h-5 rounded-full shadow-md transform transition-transform duration-200 ${
                in_ear_detection ? "translate-x-5" : "translate-x-0 bg-white"
              }`}
            />
          </button>
        </div>

        {/* Low Lag Mode Toggle */}
        <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between">
          <div>
            <h4 className="text-base font-medium text-white">Low lag mode</h4>
            <p className="text-xs text-nothing-grey mt-0.5">
              Minimizes audio latency for gaming
            </p>
          </div>
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

        {/* Ear Tip Fit Test */}
        <button
          onClick={() => setIsFitTestOpen(true)}
          className="bg-[#1c1c1e] hover:bg-[#262628] active:scale-[0.98] transition-all rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between text-left group"
        >
          <div className="flex items-center gap-3">
            <Sparkles className="w-5 h-5 text-white/80" />
            <div>
              <h4 className="text-base font-medium text-white">Ear tip fit test</h4>
              <p className="text-xs text-nothing-grey mt-0.5">
                Check acoustic seal of earbuds
              </p>
            </div>
          </div>
          <ChevronRight className="w-5 h-5 text-zinc-500 group-hover:text-white transition-colors" />
        </button>

        {/* Find My Earbuds */}
        <button
          onClick={() => setIsFindBudsOpen(true)}
          className="bg-[#1c1c1e] hover:bg-[#262628] active:scale-[0.98] transition-all rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between text-left group"
        >
          <div className="flex items-center gap-3">
            <Volume2 className="w-5 h-5 text-white/80" />
            <div>
              <h4 className="text-base font-medium text-white">Find my earbuds</h4>
              <p className="text-xs text-nothing-grey mt-0.5">
                Play sound to locate earbuds
              </p>
            </div>
          </div>
          <ChevronRight className="w-5 h-5 text-zinc-500 group-hover:text-white transition-colors" />
        </button>

        {/* Technical Info Card */}
        <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex flex-col gap-3 mt-2">
          <div className="flex justify-between items-center text-xs">
            <span className="text-zinc-400">Firmware version</span>
            <span className="font-mono text-white font-medium">
              {firmware_version || "1.0.1.37"} (Latest)
            </span>
          </div>
          <div className="flex justify-between items-center text-xs">
            <span className="text-zinc-400">Serial number</span>
            <span className="font-mono text-white font-medium">
              {serial_number || "SH247900123456"}
            </span>
          </div>
        </div>
      </div>

      {/* Modals */}
      <EarTipFitTestModal
        isOpen={isFitTestOpen}
        onClose={() => setIsFitTestOpen(false)}
      />

      <FindMyBudsModal
        isOpen={isFindBudsOpen}
        onClose={() => setIsFindBudsOpen(false)}
      />
    </div>
  );
};
