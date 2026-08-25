import { useEffect, useState } from "react";
import { Bluetooth, Check, RefreshCw, X, Radio } from "lucide-react";
import { DiscoveredDevice } from "@/types/device";
import { scanDevices } from "@/lib/tauri";
import { useDeviceStore } from "@/store/useDeviceStore";

interface DeviceScanModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const DeviceScanModal = ({ isOpen, onClose }: DeviceScanModalProps) => {
  const { device, connect } = useDeviceStore();
  const [devices, setDevices] = useState<DiscoveredDevice[]>([]);
  const [isScanning, setIsScanning] = useState(false);
  const [connectingAddress, setConnectingAddress] = useState<string | null>(null);

  const handleScan = async () => {
    setIsScanning(true);
    try {
      const results = await scanDevices();
      setDevices(results);
    } catch (e) {
      console.warn("Failed to scan devices", e);
    } finally {
      setIsScanning(false);
    }
  };

  useEffect(() => {
    if (isOpen) {
      handleScan();
    }
  }, [isOpen]);

  if (!isOpen) return null;

  const handleConnect = async (dev: DiscoveredDevice) => {
    setConnectingAddress(dev.address);
    try {
      await connect(dev.address, dev.name);
      onClose();
    } catch (e) {
      console.warn("Failed to connect", e);
    } finally {
      setConnectingAddress(null);
    }
  };

  return (
    <div className="fixed inset-0 z-50 bg-black/85 backdrop-blur-md flex items-center justify-center p-4 animate-fadeIn">
      <div className="bg-[#1c1c1e] rounded-3xl p-6 w-full max-w-sm border border-white/10 shadow-2xl flex flex-col relative max-h-[80vh]">
        {/* Close Button */}
        <button
          onClick={onClose}
          className="absolute top-4 right-4 p-1.5 rounded-full hover:bg-white/10 text-zinc-400 hover:text-white transition-colors"
          aria-label="Close"
        >
          <X className="w-5 h-5" />
        </button>

        <div className="flex items-center gap-2 mb-1">
          <Bluetooth className="w-5 h-5 text-nothing-red" />
          <h3 className="text-xl font-serif text-white">Select Device</h3>
        </div>

        <p className="text-xs text-nothing-grey mb-4">
          Scan and connect to your paired Nothing or CMF earbuds.
        </p>

        {/* Scan / Refresh Header */}
        <div className="flex items-center justify-between pb-2 mb-2 border-b border-white/5">
          <span className="text-xs font-mono text-zinc-400">
            {devices.length} device{devices.length === 1 ? "" : "s"} found
          </span>
          <button
            onClick={handleScan}
            disabled={isScanning}
            className="flex items-center gap-1.5 text-xs text-zinc-300 hover:text-white transition-colors p-1"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${isScanning ? "animate-spin text-nothing-red" : ""}`} />
            <span>{isScanning ? "Scanning..." : "Rescan"}</span>
          </button>
        </div>

        {/* Devices List */}
        <div className="flex-1 overflow-y-auto flex flex-col gap-2 my-2 pr-1">
          {devices.length === 0 && !isScanning && (
            <div className="py-8 flex flex-col items-center justify-center text-center text-zinc-500">
              <Radio className="w-8 h-8 mb-2 opacity-50" />
              <p className="text-xs">No paired Bluetooth devices detected.</p>
              <p className="text-[11px] text-zinc-600 mt-1">
                Make sure your earbuds are paired in Windows Settings.
              </p>
            </div>
          )}

          {devices.map((dev) => {
            const isCurrent = device.is_connected && device.address === dev.address;
            const isConnecting = connectingAddress === dev.address;
            const isTarget = dev.name.includes("Nothing") || dev.name.includes("CMF") || dev.name.includes("Buds") || dev.name.includes("Ear");

            return (
              <div
                key={dev.address}
                className={`p-3.5 rounded-2xl flex items-center justify-between border transition-all ${
                  isCurrent
                    ? "bg-zinc-800/80 border-white/20"
                    : isTarget
                    ? "bg-[#252528] border-white/10 hover:border-white/20"
                    : "bg-black/30 border-white/5 opacity-70"
                }`}
              >
                <div className="flex flex-col truncate pr-2">
                  <span className="text-sm font-medium text-white truncate">
                    {dev.name}
                  </span>
                  <span className="text-[11px] font-mono text-zinc-400">
                    {dev.address}
                  </span>
                </div>

                {isCurrent ? (
                  <div className="flex items-center gap-1 text-emerald-400 text-xs font-semibold px-2.5 py-1 bg-emerald-950/40 rounded-full border border-emerald-800/50">
                    <Check className="w-3.5 h-3.5" />
                    <span>Active</span>
                  </div>
                ) : (
                  <button
                    onClick={() => handleConnect(dev)}
                    disabled={isConnecting}
                    className="px-3.5 py-1.5 rounded-full text-xs font-semibold bg-white text-black hover:bg-zinc-200 active:scale-95 transition-all disabled:opacity-50"
                  >
                    {isConnecting ? "Connecting..." : "Connect"}
                  </button>
                )}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
