import { Unlink, X } from "lucide-react";

interface ActionButtonsRowProps {
  onForget?: () => void;
  onDisconnect?: () => void;
}

export const ActionButtonsRow = ({
  onForget,
  onDisconnect,
}: ActionButtonsRowProps) => {
  return (
    <div className="grid grid-cols-2 gap-3 px-4 my-3 w-full">
      {/* Forget Button */}
      <button
        onClick={onForget}
        className="flex items-center justify-center gap-2 bg-[#1c1c1e] hover:bg-[#2c2c2e] active:scale-[0.98] text-white py-4 px-4 rounded-3xl transition-all duration-150 border border-white/5 shadow-sm"
      >
        <Unlink className="w-5 h-5 text-white/80" />
        <span className="text-sm font-medium">Forget</span>
      </button>

      {/* Disconnect Button */}
      <button
        onClick={onDisconnect}
        className="flex items-center justify-center gap-2 bg-[#1c1c1e] hover:bg-[#2c2c2e] active:scale-[0.98] text-white py-4 px-4 rounded-3xl transition-all duration-150 border border-white/5 shadow-sm"
      >
        <X className="w-5 h-5 text-white/80" />
        <span className="text-sm font-medium">Disconnect</span>
      </button>
    </div>
  );
};
