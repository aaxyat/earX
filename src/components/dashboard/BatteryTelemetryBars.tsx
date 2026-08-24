import { Zap } from "lucide-react";
import { BatteryTelemetry } from "@/types/device";

interface BatteryTelemetryBarsProps {
  battery: BatteryTelemetry;
}

export const BatteryTelemetryBars = ({ battery }: BatteryTelemetryBarsProps) => {
  const leftVal = battery.left ?? 95;
  const rightVal = battery.right ?? 90;
  const caseVal = battery.case ?? 40;

  return (
    <div className="flex items-center justify-around w-full max-w-sm mx-auto px-4 py-2 my-2">
      {/* Left Earbud */}
      <div className="flex flex-col items-center gap-1.5 flex-1">
        <div className="w-16 h-1 bg-zinc-800 rounded-full overflow-hidden">
          <div
            className="h-full bg-white rounded-full transition-all duration-500"
            style={{ width: `${leftVal}%` }}
          />
        </div>
        <div className="flex items-center gap-1 text-xs font-mono text-white/90">
          {battery.is_charging_left && <Zap className="w-3 h-3 text-emerald-400 fill-emerald-400" />}
          <span>L {leftVal}%</span>
        </div>
      </div>

      {/* Case */}
      <div className="flex flex-col items-center gap-1.5 flex-1">
        <div className="w-16 h-1 bg-zinc-800 rounded-full overflow-hidden">
          <div
            className={`h-full rounded-full transition-all duration-500 ${
              battery.is_charging_case ? "bg-emerald-400" : "bg-white"
            }`}
            style={{ width: `${caseVal}%` }}
          />
        </div>
        <div className="flex items-center gap-1 text-xs font-mono text-white/90">
          {battery.is_charging_case && (
            <Zap className="w-3 h-3 text-emerald-400 fill-emerald-400" />
          )}
          <span>C {caseVal}%</span>
        </div>
      </div>

      {/* Right Earbud */}
      <div className="flex flex-col items-center gap-1.5 flex-1">
        <div className="w-16 h-1 bg-zinc-800 rounded-full overflow-hidden">
          <div
            className="h-full bg-white rounded-full transition-all duration-500"
            style={{ width: `${rightVal}%` }}
          />
        </div>
        <div className="flex items-center gap-1 text-xs font-mono text-white/90">
          {battery.is_charging_right && <Zap className="w-3 h-3 text-emerald-400 fill-emerald-400" />}
          <span>R {rightVal}%</span>
        </div>
      </div>
    </div>
  );
};
