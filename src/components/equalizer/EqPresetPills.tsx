interface EqPresetPillsProps {
  currentPreset: string;
  onSelectPreset: (preset: string) => void;
}

export const PRESETS = [
  { name: "Balanced", bass: 0, mid: 0, treble: 0 },
  { name: "More Bass", bass: 4, mid: 0, treble: -1 },
  { name: "More Treble", bass: -1, mid: 1, treble: 4 },
  { name: "Voice", bass: -2, mid: 3, treble: 1 },
  { name: "Rock", bass: 3, mid: -1, treble: 3 },
  { name: "Pop", bass: 2, mid: 2, treble: 1 },
  { name: "Custom", bass: 0, mid: 0, treble: 0 },
];

export const EqPresetPills = ({
  currentPreset,
  onSelectPreset,
}: EqPresetPillsProps) => {
  return (
    <div className="flex items-center gap-2 overflow-x-auto py-2 px-4 no-scrollbar">
      {PRESETS.map((p) => {
        const isSelected = currentPreset.toLowerCase() === p.name.toLowerCase();
        return (
          <button
            key={p.name}
            onClick={() => onSelectPreset(p.name)}
            className={`px-4 py-2 rounded-full text-xs font-medium whitespace-nowrap transition-all duration-200 active:scale-95 border ${
              isSelected
                ? "bg-white text-black border-white shadow-md font-semibold"
                : "bg-[#1c1c1e] text-zinc-400 border-white/5 hover:bg-[#2c2c2e] hover:text-white"
            }`}
          >
            {p.name}
          </button>
        );
      })}
    </div>
  );
};
