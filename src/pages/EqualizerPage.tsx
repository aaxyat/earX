import { TopHeader } from "@/components/layout/TopHeader";
import { EqPresetPills, PRESETS } from "@/components/equalizer/EqPresetPills";
import { WaveEqualizerCanvas } from "@/components/equalizer/WaveEqualizerCanvas";
import { UltraBassControl } from "@/components/equalizer/UltraBassControl";
import { useDeviceStore } from "@/store/useDeviceStore";

export const EqualizerPage = () => {
  const { device, setCustomEq, setUltraBass, setActiveTab } = useDeviceStore();
  const { eq } = device;

  const handleSelectPreset = (presetName: string) => {
    const preset = PRESETS.find((p) => p.name.toLowerCase() === presetName.toLowerCase());
    if (preset) {
      setCustomEq(preset.bass, preset.mid, preset.treble);
    }
  };

  const handleWaveChange = (bass: number, mid: number, treble: number) => {
    setCustomEq(bass, mid, treble);
  };

  return (
    <div className="flex flex-col min-h-screen bg-black text-white pb-6 animate-fadeIn">
      {/* Top Header with Back Navigation */}
      <TopHeader
        title="Equalizer"
        showBack={true}
        onBack={() => setActiveTab("dashboard")}
        showEdit={false}
      />

      <div className="flex-1 overflow-y-auto pt-2">
        {/* Preset Selector */}
        <EqPresetPills
          currentPreset={eq.preset}
          onSelectPreset={handleSelectPreset}
        />

        {/* Interactive Wave Equalizer Canvas */}
        <WaveEqualizerCanvas
          bass={eq.custom_bass}
          mid={eq.custom_mid}
          treble={eq.custom_treble}
          onChange={handleWaveChange}
        />

        {/* Ultra Bass Control */}
        <UltraBassControl
          enabled={eq.ultra_bass_enabled}
          level={eq.ultra_bass_level}
          onToggle={(enabled) => setUltraBass(enabled, eq.ultra_bass_level || 2)}
          onSetLevel={(level) => setUltraBass(true, level)}
        />
      </div>
    </div>
  );
};
