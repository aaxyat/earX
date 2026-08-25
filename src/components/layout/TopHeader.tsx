import { ArrowLeft, Bluetooth, Edit2 } from "lucide-react";

interface TopHeaderProps {
  title?: string;
  onBack?: () => void;
  showBack?: boolean;
  onEdit?: () => void;
  showEdit?: boolean;
  onScan?: () => void;
  showScan?: boolean;
  isConnected?: boolean;
}

export const TopHeader = ({
  title = "Device details",
  onBack,
  showBack = false,
  onEdit,
  showEdit = true,
  onScan,
  showScan = false,
  isConnected = true,
}: TopHeaderProps) => {
  return (
    <header className="sticky top-0 z-30 bg-black/90 backdrop-blur-md px-5 py-4 flex items-center justify-between border-b border-white/5">
      <div className="flex items-center gap-3">
        {showBack ? (
          <button
            onClick={onBack}
            className="p-2 -ml-2 rounded-full hover:bg-white/10 active:scale-95 transition-all text-white"
            aria-label="Back"
          >
            <ArrowLeft className="w-6 h-6" />
          </button>
        ) : (
          <button
            onClick={onScan}
            className="p-2 -ml-2 rounded-full hover:bg-white/10 active:scale-95 transition-all text-white relative group"
            aria-label="Bluetooth Devices"
            title="Scan Bluetooth Devices"
          >
            <Bluetooth className="w-5 h-5 text-white/90 group-hover:text-white" />
            <span
              className={`absolute top-1.5 right-1.5 w-2 h-2 rounded-full ${
                isConnected ? "bg-emerald-400" : "bg-zinc-600"
              }`}
            />
          </button>
        )}
        <h1 className="text-2xl font-serif tracking-tight text-white select-none">
          {title}
        </h1>
      </div>

      <div className="flex items-center gap-1">
        {showScan && (
          <button
            onClick={onScan}
            className="p-2 rounded-full hover:bg-white/10 active:scale-95 transition-all text-white/90"
            aria-label="Scan Devices"
            title="Scan Bluetooth Devices"
          >
            <Bluetooth className="w-5 h-5" />
          </button>
        )}

        {showEdit && (
          <button
            onClick={onEdit}
            className="p-2 rounded-full hover:bg-white/10 active:scale-95 transition-all text-white/90"
            aria-label="Edit device details"
          >
            <Edit2 className="w-5 h-5" />
          </button>
        )}
      </div>
    </header>
  );
};
