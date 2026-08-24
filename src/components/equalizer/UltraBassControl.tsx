interface UltraBassControlProps {
  enabled: boolean;
  level: number;
  onToggle: (enabled: boolean) => void;
  onSetLevel: (level: number) => void;
}

export const UltraBassControl = ({
  enabled,
  level,
  onToggle,
  onSetLevel,
}: UltraBassControlProps) => {
  return (
    <div className="bg-[#1c1c1e] rounded-3xl p-5 mx-4 my-2 border border-white/5 shadow-md">
      <div className="flex items-center justify-between mb-4">
        <div>
          <h4 className="text-base font-medium text-white">Ultra bass</h4>
          <p className="text-xs text-nothing-grey mt-0.5">
            {enabled ? `On · Level ${level}` : "Off"}
          </p>
        </div>

        {/* Toggle Switch */}
        <button
          onClick={() => onToggle(!enabled)}
          className={`w-12 h-7 flex items-center rounded-full p-1 transition-colors duration-200 ${
            enabled ? "bg-white" : "bg-zinc-800"
          }`}
          aria-label="Toggle Ultra Bass"
        >
          <div
            className={`bg-black w-5 h-5 rounded-full shadow-md transform transition-transform duration-200 ${
              enabled ? "translate-x-5" : "translate-x-0 bg-white"
            }`}
          />
        </button>
      </div>

      {/* Discrete 5-Level Selector Pills */}
      {enabled && (
        <div className="pt-2 border-t border-white/5">
          <div className="flex items-center justify-between gap-2">
            {[1, 2, 3, 4, 5].map((lvl) => {
              const isSelected = level === lvl;
              return (
                <button
                  key={lvl}
                  onClick={() => onSetLevel(lvl)}
                  className={`flex-1 py-2.5 rounded-2xl text-xs font-mono font-semibold transition-all duration-150 active:scale-95 border ${
                    isSelected
                      ? "bg-nothing-red text-white border-nothing-red shadow-[0_0_10px_rgba(215,25,32,0.5)]"
                      : "bg-[#2c2c2e] text-zinc-400 border-white/5 hover:bg-[#38383a] hover:text-white"
                  }`}
                >
                  {lvl}
                </button>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
};
