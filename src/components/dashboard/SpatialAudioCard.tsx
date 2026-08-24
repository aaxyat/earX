import { MoreHorizontal } from "lucide-react";

interface SpatialAudioCardProps {
  spatialAudio: boolean;
  onToggle: (enabled: boolean) => void;
}

export const SpatialAudioCard = ({
  spatialAudio,
  onToggle,
}: SpatialAudioCardProps) => {
  return (
    <div className="bg-[#1c1c1e] rounded-3xl p-5 mx-4 my-2 border border-white/5 shadow-md">
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-base font-medium text-white">Spatial audio</h3>
        <button
          className="p-1 -mr-1 rounded-full text-zinc-400 hover:text-white hover:bg-white/10 transition-colors"
          aria-label="Spatial audio settings"
        >
          <MoreHorizontal className="w-5 h-5" />
        </button>
      </div>

      {/* Mode Switcher */}
      <div className="grid grid-cols-2 gap-4 items-center justify-items-center">
        {/* Fixed */}
        <button
          onClick={() => onToggle(true)}
          className="flex flex-col items-center gap-2 group"
        >
          <div
            className={`w-16 h-16 rounded-full flex items-center justify-center transition-all duration-200 active:scale-95 ${
              spatialAudio
                ? "bg-white text-black shadow-lg"
                : "bg-[#2c2c2e] text-zinc-400 hover:bg-[#38383a]"
            }`}
          >
            <svg
              className="w-7 h-7"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <circle cx="12" cy="12" r="2" fill="currentColor" />
              <path d="M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49" />
            </svg>
          </div>
          <span
            className={`text-xs font-medium ${
              spatialAudio ? "text-white" : "text-zinc-400"
            }`}
          >
            Fixed
          </span>
        </button>

        {/* Off */}
        <button
          onClick={() => onToggle(false)}
          className="flex flex-col items-center gap-2 group"
        >
          <div
            className={`w-16 h-16 rounded-full flex items-center justify-center transition-all duration-200 active:scale-95 ${
              !spatialAudio
                ? "bg-white text-black shadow-lg"
                : "bg-[#2c2c2e] text-zinc-400 hover:bg-[#38383a]"
            }`}
          >
            <svg
              className="w-7 h-7"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <circle cx="12" cy="12" r="3" fill="currentColor" />
              <path d="M19 12a7 7 0 0 0-14 0" />
            </svg>
          </div>
          <span
            className={`text-xs font-medium ${
              !spatialAudio ? "text-white" : "text-zinc-400"
            }`}
          >
            Off
          </span>
        </button>
      </div>
    </div>
  );
};
