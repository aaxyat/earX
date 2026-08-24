import { useState } from "react";
import { Bell, Volume2, X } from "lucide-react";
import { useDeviceStore } from "@/store/useDeviceStore";

interface FindMyBudsModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const FindMyBudsModal = ({
  isOpen,
  onClose,
}: FindMyBudsModalProps) => {
  const { ringBuds } = useDeviceStore();
  const [ringingLeft, setRingingLeft] = useState(false);
  const [ringingRight, setRingingRight] = useState(false);

  if (!isOpen) return null;

  const handleToggleLeft = () => {
    const next = !ringingLeft;
    setRingingLeft(next);
    ringBuds(true, next);
  };

  const handleToggleRight = () => {
    const next = !ringingRight;
    setRingingRight(next);
    ringBuds(false, next);
  };

  return (
    <div className="fixed inset-0 z-50 bg-black/85 backdrop-blur-md flex items-center justify-center p-4">
      <div className="bg-[#1c1c1e] rounded-3xl p-6 w-full max-w-sm border border-white/10 shadow-2xl flex flex-col relative">
        {/* Close Button */}
        <button
          onClick={() => {
            if (ringingLeft) ringBuds(true, false);
            if (ringingRight) ringBuds(false, false);
            onClose();
          }}
          className="absolute top-4 right-4 p-1.5 rounded-full hover:bg-white/10 text-zinc-400 hover:text-white transition-colors"
          aria-label="Close"
        >
          <X className="w-5 h-5" />
        </button>

        <div className="flex items-center gap-2 mb-2">
          <Bell className="w-5 h-5 text-nothing-red" />
          <h3 className="text-xl font-serif text-white">Find my earbuds</h3>
        </div>

        <p className="text-xs text-nothing-grey mb-6">
          Plays a high-pitched acoustic sound to help locate your earbuds. Do NOT play while wearing them.
        </p>

        {/* Ring Controls */}
        <div className="grid grid-cols-2 gap-3 my-2">
          {/* Left Bud */}
          <button
            onClick={handleToggleLeft}
            className={`p-4 rounded-2xl flex flex-col items-center gap-3 transition-all duration-200 active:scale-95 border ${
              ringingLeft
                ? "bg-nothing-darkRed text-white border-nothing-red shadow-[0_0_15px_rgba(215,25,32,0.6)] animate-pulse"
                : "bg-black/40 text-zinc-300 border-white/5 hover:bg-[#2c2c2e]"
            }`}
          >
            <Volume2 className={`w-7 h-7 ${ringingLeft ? "text-white" : "text-zinc-400"}`} />
            <div className="flex flex-col items-center">
              <span className="text-sm font-semibold">Left Earbud</span>
              <span className="text-xs text-zinc-400 mt-0.5">
                {ringingLeft ? "STOPPING" : "Play Sound"}
              </span>
            </div>
          </button>

          {/* Right Bud */}
          <button
            onClick={handleToggleRight}
            className={`p-4 rounded-2xl flex flex-col items-center gap-3 transition-all duration-200 active:scale-95 border ${
              ringingRight
                ? "bg-nothing-darkRed text-white border-nothing-red shadow-[0_0_15px_rgba(215,25,32,0.6)] animate-pulse"
                : "bg-black/40 text-zinc-300 border-white/5 hover:bg-[#2c2c2e]"
            }`}
          >
            <Volume2 className={`w-7 h-7 ${ringingRight ? "text-white" : "text-zinc-400"}`} />
            <div className="flex flex-col items-center">
              <span className="text-sm font-semibold">Right Earbud</span>
              <span className="text-xs text-zinc-400 mt-0.5">
                {ringingRight ? "STOPPING" : "Play Sound"}
              </span>
            </div>
          </button>
        </div>
      </div>
    </div>
  );
};
