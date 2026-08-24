import { useState } from "react";
import { CheckCircle2, Play, RefreshCw, X } from "lucide-react";

interface EarTipFitTestModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const EarTipFitTestModal = ({
  isOpen,
  onClose,
}: EarTipFitTestModalProps) => {
  const [isRunning, setIsRunning] = useState(false);
  const [result, setResult] = useState<{ left: "Good" | "Adjust"; right: "Good" | "Adjust" } | null>(null);

  if (!isOpen) return null;

  const handleStartTest = () => {
    setIsRunning(true);
    setResult(null);
    setTimeout(() => {
      setIsRunning(false);
      setResult({ left: "Good", right: "Good" });
    }, 2500);
  };

  return (
    <div className="fixed inset-0 z-50 bg-black/85 backdrop-blur-md flex items-center justify-center p-4">
      <div className="bg-[#1c1c1e] rounded-3xl p-6 w-full max-w-sm border border-white/10 shadow-2xl flex flex-col relative">
        {/* Close Button */}
        <button
          onClick={onClose}
          className="absolute top-4 right-4 p-1.5 rounded-full hover:bg-white/10 text-zinc-400 hover:text-white transition-colors"
          aria-label="Close"
        >
          <X className="w-5 h-5" />
        </button>

        <h3 className="text-xl font-serif text-white mb-2">Ear tip fit test</h3>
        <p className="text-xs text-nothing-grey mb-6">
          Put both earbuds in your ears and stay in a quiet environment before starting the test.
        </p>

        {/* Visualizer & Results */}
        <div className="flex items-center justify-around my-4 py-4 bg-black/40 rounded-2xl border border-white/5">
          {/* Left Bud */}
          <div className="flex flex-col items-center gap-2">
            <span className="text-xs font-mono font-medium text-zinc-400">Left Ear</span>
            {result ? (
              <div className="flex items-center gap-1 text-emerald-400 text-xs font-semibold">
                <CheckCircle2 className="w-4 h-4" />
                <span>Good seal</span>
              </div>
            ) : (
              <span className="text-xs text-zinc-600">Ready</span>
            )}
          </div>

          {/* Right Bud */}
          <div className="flex flex-col items-center gap-2">
            <span className="text-xs font-mono font-medium text-zinc-400">Right Ear</span>
            {result ? (
              <div className="flex items-center gap-1 text-emerald-400 text-xs font-semibold">
                <CheckCircle2 className="w-4 h-4" />
                <span>Good seal</span>
              </div>
            ) : (
              <span className="text-xs text-zinc-600">Ready</span>
            )}
          </div>
        </div>

        {/* Action Button */}
        <button
          onClick={handleStartTest}
          disabled={isRunning}
          className={`w-full py-3.5 mt-4 rounded-full font-medium text-sm flex items-center justify-center gap-2 transition-all active:scale-95 ${
            isRunning
              ? "bg-zinc-800 text-zinc-400 cursor-not-allowed"
              : "bg-white text-black hover:bg-zinc-100 shadow-md font-semibold"
          }`}
        >
          {isRunning ? (
            <>
              <RefreshCw className="w-4 h-4 animate-spin" />
              <span>Testing acoustic seal...</span>
            </>
          ) : result ? (
            <>
              <RefreshCw className="w-4 h-4" />
              <span>Test Again</span>
            </>
          ) : (
            <>
              <Play className="w-4 h-4 fill-black" />
              <span>Start Test</span>
            </>
          )}
        </button>
      </div>
    </div>
  );
};
