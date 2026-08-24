import { useState } from "react";
import { TopHeader } from "@/components/layout/TopHeader";
import { useDeviceStore } from "@/store/useDeviceStore";

const GESTURE_ACTIONS = [
  "Play/Pause",
  "Next Track",
  "Previous Track",
  "Noise control",
  "Voice Assistant",
  "Volume up",
  "Volume down",
  "No action",
];

export const GesturesPage = () => {
  const { setActiveTab } = useDeviceStore();
  const [selectedEar, setSelectedEar] = useState<"left" | "right">("left");

  // Local gesture mapping state
  const [gestures, setGestures] = useState({
    left: {
      doubleTap: "Next Track",
      tripleTap: "Previous Track",
      tapAndHold: "Noise control",
      doubleTapAndHold: "Voice Assistant",
    },
    right: {
      doubleTap: "Next Track",
      tripleTap: "Previous Track",
      tapAndHold: "Noise control",
      doubleTapAndHold: "Voice Assistant",
    },
  });

  const updateGesture = (
    gestureKey: keyof typeof gestures.left,
    action: string
  ) => {
    setGestures((prev) => ({
      ...prev,
      [selectedEar]: {
        ...prev[selectedEar],
        [gestureKey]: action,
      },
    }));
  };

  const currentMap = gestures[selectedEar];

  return (
    <div className="flex flex-col min-h-screen bg-black text-white pb-6 animate-fadeIn">
      {/* Top Header */}
      <TopHeader
        title="Controls"
        showBack={true}
        onBack={() => setActiveTab("dashboard")}
        showEdit={false}
      />

      <div className="flex-1 overflow-y-auto px-4 pt-2">
        {/* Left / Right Ear Switcher */}
        <div className="grid grid-cols-2 gap-2 bg-[#1c1c1e] p-1.5 rounded-full border border-white/5 my-3">
          <button
            onClick={() => setSelectedEar("left")}
            className={`py-2.5 rounded-full text-xs font-semibold transition-all ${
              selectedEar === "left"
                ? "bg-white text-black shadow-md"
                : "text-zinc-400 hover:text-white"
            }`}
          >
            Left Earbud
          </button>
          <button
            onClick={() => setSelectedEar("right")}
            className={`py-2.5 rounded-full text-xs font-semibold transition-all ${
              selectedEar === "right"
                ? "bg-white text-black shadow-md"
                : "text-zinc-400 hover:text-white"
            }`}
          >
            Right Earbud
          </button>
        </div>

        {/* Gesture Rows */}
        <div className="flex flex-col gap-3 mt-4">
          {/* Double Tap */}
          <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between">
            <div>
              <h4 className="text-sm font-medium text-white">Double tap</h4>
              <p className="text-xs text-nothing-grey mt-0.5">{currentMap.doubleTap}</p>
            </div>
            <select
              value={currentMap.doubleTap}
              onChange={(e) => updateGesture("doubleTap", e.target.value)}
              className="bg-zinc-800 text-xs text-white rounded-xl px-3 py-2 border border-white/10 focus:outline-none focus:ring-1 focus:ring-white"
            >
              {GESTURE_ACTIONS.map((action) => (
                <option key={action} value={action}>
                  {action}
                </option>
              ))}
            </select>
          </div>

          {/* Triple Tap */}
          <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between">
            <div>
              <h4 className="text-sm font-medium text-white">Triple tap</h4>
              <p className="text-xs text-nothing-grey mt-0.5">{currentMap.tripleTap}</p>
            </div>
            <select
              value={currentMap.tripleTap}
              onChange={(e) => updateGesture("tripleTap", e.target.value)}
              className="bg-zinc-800 text-xs text-white rounded-xl px-3 py-2 border border-white/10 focus:outline-none focus:ring-1 focus:ring-white"
            >
              {GESTURE_ACTIONS.map((action) => (
                <option key={action} value={action}>
                  {action}
                </option>
              ))}
            </select>
          </div>

          {/* Tap & Hold */}
          <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between">
            <div>
              <h4 className="text-sm font-medium text-white">Tap & hold</h4>
              <p className="text-xs text-nothing-grey mt-0.5">{currentMap.tapAndHold}</p>
            </div>
            <select
              value={currentMap.tapAndHold}
              onChange={(e) => updateGesture("tapAndHold", e.target.value)}
              className="bg-zinc-800 text-xs text-white rounded-xl px-3 py-2 border border-white/10 focus:outline-none focus:ring-1 focus:ring-white"
            >
              {GESTURE_ACTIONS.map((action) => (
                <option key={action} value={action}>
                  {action}
                </option>
              ))}
            </select>
          </div>

          {/* Double Tap & Hold */}
          <div className="bg-[#1c1c1e] rounded-3xl p-5 border border-white/5 shadow-md flex items-center justify-between">
            <div>
              <h4 className="text-sm font-medium text-white">Double tap & hold</h4>
              <p className="text-xs text-nothing-grey mt-0.5">
                {currentMap.doubleTapAndHold}
              </p>
            </div>
            <select
              value={currentMap.doubleTapAndHold}
              onChange={(e) => updateGesture("doubleTapAndHold", e.target.value)}
              className="bg-zinc-800 text-xs text-white rounded-xl px-3 py-2 border border-white/10 focus:outline-none focus:ring-1 focus:ring-white"
            >
              {GESTURE_ACTIONS.map((action) => (
                <option key={action} value={action}>
                  {action}
                </option>
              ))}
            </select>
          </div>
        </div>
      </div>
    </div>
  );
};
