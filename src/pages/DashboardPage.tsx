import { useEffect, useState } from "react";
import { TopHeader } from "@/components/layout/TopHeader";
import { DualEarbudsVisualizer } from "@/components/dashboard/DualEarbudsVisualizer";
import { BatteryTelemetryBars } from "@/components/dashboard/BatteryTelemetryBars";
import { ActionButtonsRow } from "@/components/dashboard/ActionButtonsRow";
import { NoiseCancellationCard } from "@/components/dashboard/NoiseCancellationCard";
import { SpatialAudioCard } from "@/components/dashboard/SpatialAudioCard";
import { BentoCardGrid } from "@/components/dashboard/BentoCardGrid";
import { useDeviceStore } from "@/store/useDeviceStore";

export const DashboardPage = () => {
  const {
    device,
    init,
    setAnc,
    setSpatialAudio,
    disconnect,
    setActiveTab,
    setDeviceName,
  } = useDeviceStore();

  const [isRenaming, setIsRenaming] = useState(false);
  const [nameInput, setNameInput] = useState(device.device_name);

  useEffect(() => {
    init();
  }, [init]);

  const handleSaveName = () => {
    if (nameInput.trim()) {
      setDeviceName(nameInput.trim());
    }
    setIsRenaming(false);
  };

  const leftImg = device.model?.left_image || "/assets/espeon_blue_left.webp";
  const rightImg = device.model?.right_image || "/assets/espeon_blue_right.webp";

  return (
    <div className="flex flex-col min-h-screen bg-black text-white pb-6 animate-fadeIn">
      {/* Top Header */}
      <TopHeader
        title="Device details"
        showBack={false}
        showEdit={true}
        onEdit={() => {
          setNameInput(device.device_name);
          setIsRenaming(true);
        }}
      />

      {/* Rename Dialog Modal */}
      {isRenaming && (
        <div className="fixed inset-0 z-50 bg-black/80 backdrop-blur-sm flex items-center justify-center p-4">
          <div className="bg-[#1c1c1e] rounded-3xl p-6 w-full max-w-xs border border-white/10 shadow-2xl">
            <h3 className="text-lg font-serif mb-3">Rename Device</h3>
            <input
              type="text"
              value={nameInput}
              onChange={(e) => setNameInput(e.target.value)}
              className="w-full bg-zinc-800 text-white rounded-xl px-3 py-2 text-sm focus:outline-none focus:ring-1 focus:ring-white mb-4"
              autoFocus
            />
            <div className="flex items-center justify-end gap-2">
              <button
                onClick={() => setIsRenaming(false)}
                className="px-4 py-2 text-xs font-medium text-zinc-400 hover:text-white"
              >
                Cancel
              </button>
              <button
                onClick={handleSaveName}
                className="px-4 py-2 text-xs font-semibold bg-white text-black rounded-full active:scale-95"
              >
                Save
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Main Scroll Content */}
      <div className="flex-1 overflow-y-auto">
        {/* Hero Visualizer */}
        <DualEarbudsVisualizer
          deviceName={device.device_name}
          leftImage={leftImg}
          rightImage={rightImg}
        />

        {/* Battery Telemetry Row */}
        <BatteryTelemetryBars battery={device.battery} />

        {/* Action Buttons: Forget & Disconnect */}
        <ActionButtonsRow onDisconnect={disconnect} />

        {/* Noise Cancellation Card with 3 circular dials and 4-step segmented pill */}
        <NoiseCancellationCard
          ancMode={device.anc_mode}
          onSetAnc={setAnc}
        />

        {/* Spatial Audio Card */}
        <SpatialAudioCard
          spatialAudio={device.spatial_audio}
          onToggle={setSpatialAudio}
        />

        {/* 2-Column Bento Card Grid */}
        <BentoCardGrid onNavigate={setActiveTab} />
      </div>
    </div>
  );
};
