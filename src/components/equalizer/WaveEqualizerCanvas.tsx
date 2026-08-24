import React, { useRef, useState } from "react";

interface WaveEqualizerCanvasProps {
  bass: number;
  mid: number;
  treble: number;
  onChange: (bass: number, mid: number, treble: number) => void;
}

export const WaveEqualizerCanvas = ({
  bass,
  mid,
  treble,
  onChange,
}: WaveEqualizerCanvasProps) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [draggingNode, setDraggingNode] = useState<"bass" | "mid" | "treble" | null>(null);

  const width = 360;
  const height = 200;
  const padding = 30;

  // Convert dB (-6 to +6) to SVG Y coordinate
  const dbToY = (db: number) => {
    const clamped = Math.max(-6, Math.min(6, db));
    // -6 is at bottom (height - padding), +6 is at top (padding), 0 is middle
    const percent = (clamped + 6) / 12; // 0 to 1
    return height - padding - percent * (height - 2 * padding);
  };

  // Convert client Y to dB (-6 to +6)
  const clientYToDb = (clientY: number) => {
    if (!containerRef.current) return 0;
    const rect = containerRef.current.getBoundingClientRect();
    const y = clientY - rect.top;
    const effectiveHeight = height - 2 * padding;
    const normalizedY = (y / rect.height) * height;
    const percent = 1 - (normalizedY - padding) / effectiveHeight;
    const db = percent * 12 - 6;
    return Math.round(Math.max(-6, Math.min(6, db)) * 10) / 10;
  };

  const xBass = 70;
  const xMid = 180;
  const xTreble = 290;

  const yBass = dbToY(bass);
  const yMid = dbToY(mid);
  const yTreble = dbToY(treble);

  // SVG Smooth Bezier Path
  const pathD = `
    M 0 ${yBass}
    C ${xBass / 2} ${yBass}, ${xBass - 30} ${yBass}, ${xBass} ${yBass}
    C ${xBass + 40} ${yBass}, ${xMid - 40} ${yMid}, ${xMid} ${yMid}
    C ${xMid + 40} ${yMid}, ${xTreble - 40} ${yTreble}, ${xTreble} ${yTreble}
    C ${xTreble + 30} ${yTreble}, ${width - 20} ${yTreble}, ${width} ${yTreble}
  `;

  const fillD = `${pathD} L ${width} ${height} L 0 ${height} Z`;

  const handlePointerDown = (node: "bass" | "mid" | "treble") => (e: React.PointerEvent) => {
    e.preventDefault();
    setDraggingNode(node);
  };

  const handlePointerMove = (e: React.PointerEvent) => {
    if (!draggingNode) return;
    const newDb = clientYToDb(e.clientY);
    if (draggingNode === "bass") onChange(newDb, mid, treble);
    if (draggingNode === "mid") onChange(bass, newDb, treble);
    if (draggingNode === "treble") onChange(bass, mid, newDb);
  };

  const handlePointerUp = () => {
    setDraggingNode(null);
  };

  return (
    <div
      ref={containerRef}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerLeave={handlePointerUp}
      className="bg-[#1c1c1e] rounded-3xl p-5 mx-4 my-3 border border-white/5 shadow-lg relative overflow-hidden select-none"
    >
      <div className="flex justify-between items-center mb-2">
        <span className="text-xs font-mono text-zinc-500">+6 dB</span>
        <span className="text-xs font-mono text-zinc-500">0 dB</span>
        <span className="text-xs font-mono text-zinc-500">-6 dB</span>
      </div>

      <svg
        viewBox={`0 0 ${width} ${height}`}
        className="w-full h-48 overflow-visible"
      >
        <defs>
          <linearGradient id="eqGradient" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor="#d71920" stopOpacity="0.4" />
            <stop offset="50%" stopColor="#d71920" stopOpacity="0.1" />
            <stop offset="100%" stopColor="#d71920" stopOpacity="0" />
          </linearGradient>
        </defs>

        {/* Horizontal Grid lines */}
        <line
          x1="0"
          y1={dbToY(6)}
          x2={width}
          y2={dbToY(6)}
          stroke="rgba(255,255,255,0.05)"
          strokeDasharray="4 4"
        />
        <line
          x1="0"
          y1={dbToY(0)}
          x2={width}
          y2={dbToY(0)}
          stroke="rgba(255,255,255,0.15)"
        />
        <line
          x1="0"
          y1={dbToY(-6)}
          x2={width}
          y2={dbToY(-6)}
          stroke="rgba(255,255,255,0.05)"
          strokeDasharray="4 4"
        />

        {/* Shaded Area Under Curve */}
        <path d={fillD} fill="url(#eqGradient)" />

        {/* Dynamic Curve Line */}
        <path
          d={pathD}
          fill="none"
          stroke="#d71920"
          strokeWidth="3"
          strokeLinecap="round"
        />

        {/* Bass Node */}
        <g
          className="cursor-ns-resize"
          onPointerDown={handlePointerDown("bass")}
        >
          <circle
            cx={xBass}
            cy={yBass}
            r="10"
            fill="#ffffff"
            className="drop-shadow-md active:scale-125 transition-transform"
          />
          <circle cx={xBass} cy={yBass} r="4" fill="#d71920" />
        </g>

        {/* Mid Node */}
        <g
          className="cursor-ns-resize"
          onPointerDown={handlePointerDown("mid")}
        >
          <circle
            cx={xMid}
            cy={yMid}
            r="10"
            fill="#ffffff"
            className="drop-shadow-md active:scale-125 transition-transform"
          />
          <circle cx={xMid} cy={yMid} r="4" fill="#d71920" />
        </g>

        {/* Treble Node */}
        <g
          className="cursor-ns-resize"
          onPointerDown={handlePointerDown("treble")}
        >
          <circle
            cx={xTreble}
            cy={yTreble}
            r="10"
            fill="#ffffff"
            className="drop-shadow-md active:scale-125 transition-transform"
          />
          <circle cx={xTreble} cy={yTreble} r="4" fill="#d71920" />
        </g>
      </svg>

      {/* Gain Indicators */}
      <div className="grid grid-cols-3 gap-2 mt-4 pt-3 border-t border-white/5 text-center">
        <div className="flex flex-col">
          <span className="text-xs text-zinc-400">Bass</span>
          <span className="text-sm font-mono font-semibold text-white">
            {bass > 0 ? `+${bass}` : bass} dB
          </span>
        </div>
        <div className="flex flex-col">
          <span className="text-xs text-zinc-400">Mid</span>
          <span className="text-sm font-mono font-semibold text-white">
            {mid > 0 ? `+${mid}` : mid} dB
          </span>
        </div>
        <div className="flex flex-col">
          <span className="text-xs text-zinc-400">Treble</span>
          <span className="text-sm font-mono font-semibold text-white">
            {treble > 0 ? `+${treble}` : treble} dB
          </span>
        </div>
      </div>
    </div>
  );
};
