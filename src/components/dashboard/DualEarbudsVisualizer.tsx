interface DualEarbudsVisualizerProps {
  deviceName: string;
  leftImage?: string;
  rightImage?: string;
}

export const DualEarbudsVisualizer = ({
  deviceName,
  leftImage = "/assets/espeon_blue_left.webp",
  rightImage = "/assets/espeon_blue_right.webp",
}: DualEarbudsVisualizerProps) => {
  return (
    <section className="flex flex-col items-center justify-center pt-2 pb-4">
      {/* Earbuds Render */}
      <div className="flex items-center justify-center gap-6 h-56 w-full relative">
        <img
          src={leftImage}
          alt="Left Earbud"
          className="h-48 w-auto object-contain drop-shadow-[0_15px_25px_rgba(0,0,0,0.8)] transform -rotate-3 transition-transform duration-300 hover:scale-105"
        />
        <img
          src={rightImage}
          alt="Right Earbud"
          className="h-48 w-auto object-contain drop-shadow-[0_15px_25px_rgba(0,0,0,0.8)] transform rotate-3 transition-transform duration-300 hover:scale-105"
        />
      </div>

      {/* Device Name */}
      <h2 className="text-2xl font-serif font-normal tracking-tight text-white mt-3 text-center px-4">
        {deviceName}
      </h2>
    </section>
  );
};
