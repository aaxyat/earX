import { AncLevel, AncMode } from "@/types/device";

interface NoiseCancellationCardProps {
  ancMode: AncMode;
  onSetAnc: (mode: AncMode) => void;
}

export const NoiseCancellationCard = ({
  ancMode,
  onSetAnc,
}: NoiseCancellationCardProps) => {
  const isAncOn = ancMode.type === "NoiseCancellation";
  const isTransparency = ancMode.type === "Transparency";
  const isOff = ancMode.type === "Off";

  const currentLevel: AncLevel = isAncOn ? ancMode.level : "High";

  // Subtitle generation matching Nothing X app
  const getSubtitle = () => {
    if (isAncOn) return `On · ${currentLevel}`;
    if (isTransparency) return "Transparency";
    return "Off";
  };

  const ancLevels: AncLevel[] = ["Low", "Mid", "High", "Adaptive"];

  return (
    <div className="bg-[#1c1c1e] rounded-3xl p-5 mx-4 my-2 border border-white/5 shadow-md transition-all">
      {/* Card Header */}
      <div className="flex flex-col mb-4">
        <h3 className="text-base font-medium text-white">Noise cancellation</h3>
        <p className="text-xs text-nothing-grey mt-0.5">{getSubtitle()}</p>
      </div>

      {/* 3 Mode Circular Dials */}
      <div className="grid grid-cols-3 gap-2 items-center justify-items-center mb-4">
        {/* Noise Cancellation */}
        <button
          onClick={() =>
            onSetAnc({ type: "NoiseCancellation", level: currentLevel })
          }
          className="flex flex-col items-center gap-2 group"
        >
          <div
            className={`w-16 h-16 rounded-full flex items-center justify-center transition-all duration-200 active:scale-95 ${
              isAncOn
                ? "bg-white text-black shadow-lg"
                : "bg-[#2c2c2e] text-zinc-400 hover:bg-[#38383a]"
            }`}
          >
            {/* Custom Arch with Dot Icon */}
            <svg
              className="w-7 h-7"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <path d="M5 13a7 7 0 0 1 14 0" />
              <circle cx="12" cy="13" r="1.5" fill="currentColor" />
            </svg>
          </div>
          <span
            className={`text-xs font-medium ${
              isAncOn ? "text-white" : "text-zinc-400"
            }`}
          >
            Noise cancellation
          </span>
        </button>

        {/* Transparency */}
        <button
          onClick={() => onSetAnc({ type: "Transparency" })}
          className="flex flex-col items-center gap-2 group"
        >
          <div
            className={`w-16 h-16 rounded-full flex items-center justify-center transition-all duration-200 active:scale-95 ${
              isTransparency
                ? "bg-white text-black shadow-lg"
                : "bg-[#2c2c2e] text-zinc-400 hover:bg-[#38383a]"
            }`}
          >
            {/* Custom Radiating Dots Icon */}
            <svg
              className="w-7 h-7"
              viewBox="0 0 24 24"
              fill="currentColor"
            >
              <circle cx="12" cy="12" r="1.5" />
              <circle cx="12" cy="6" r="1.2" />
              <circle cx="16.2" cy="7.8" r="1.2" />
              <circle cx="18" cy="12" r="1.2" />
              <circle cx="7.8" cy="7.8" r="1.2" />
              <circle cx="6" cy="12" r="1.2" />
            </svg>
          </div>
          <span
            className={`text-xs font-medium ${
              isTransparency ? "text-white" : "text-zinc-400"
            }`}
          >
            Transparency
          </span>
        </button>

        {/* Off */}
        <button
          onClick={() => onSetAnc({ type: "Off" })}
          className="flex flex-col items-center gap-2 group"
        >
          <div
            className={`w-16 h-16 rounded-full flex items-center justify-center transition-all duration-200 active:scale-95 ${
              isOff
                ? "bg-white text-black shadow-lg"
                : "bg-[#2c2c2e] text-zinc-400 hover:bg-[#38383a]"
            }`}
          >
            {/* Slashed Circle Icon */}
            <svg
              className="w-7 h-7"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <circle cx="12" cy="12" r="9" />
              <line x1="4.93" y1="4.93" x2="19.07" y2="19.07" />
            </svg>
          </div>
          <span
            className={`text-xs font-medium ${
              isOff ? "text-white" : "text-zinc-400"
            }`}
          >
            Off
          </span>
        </button>
      </div>

      {/* 4-Step Segmented Pill Selector (Visible when ANC is active) */}
      {isAncOn && (
        <div className="pt-3 border-t border-white/5 animate-fadeIn">
          <div className="grid grid-cols-4 gap-2 text-center">
            {ancLevels.map((lvl) => {
              const isSelected = currentLevel === lvl;
              return (
                <button
                  key={lvl}
                  onClick={() =>
                    onSetAnc({ type: "NoiseCancellation", level: lvl })
                  }
                  className="flex flex-col items-center gap-1.5 py-1 focus:outline-none"
                >
                  <div
                    className={`w-full h-1.5 rounded-full transition-all duration-300 ${
                      isSelected ? "bg-white shadow-[0_0_8px_rgba(255,255,255,0.6)]" : "bg-zinc-800"
                    }`}
                  />
                  <span
                    className={`text-xs font-medium transition-colors ${
                      isSelected ? "text-white font-semibold" : "text-zinc-500"
                    }`}
                  >
                    {lvl}
                  </span>
                </button>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};
