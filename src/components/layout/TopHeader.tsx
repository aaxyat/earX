import { ArrowLeft, Edit2 } from "lucide-react";

interface TopHeaderProps {
  title?: string;
  onBack?: () => void;
  showBack?: boolean;
  onEdit?: () => void;
  showEdit?: boolean;
}

export const TopHeader = ({
  title = "Device details",
  onBack,
  showBack = false,
  onEdit,
  showEdit = true,
}: TopHeaderProps) => {
  return (
    <header className="sticky top-0 z-30 bg-black/90 backdrop-blur-md px-5 py-4 flex items-center justify-between border-b border-white/5">
      <div className="flex items-center gap-3">
        {showBack ? (
          <button
            onClick={onBack}
            className="p-2 -ml-2 rounded-full hover:bg-white/10 active:scale-95 transition-all text-white"
            aria-label="Back"
          >
            <ArrowLeft className="w-6 h-6" />
          </button>
        ) : (
          <button
            className="p-2 -ml-2 rounded-full hover:bg-white/10 active:scale-95 transition-all text-white"
            aria-label="Back"
          >
            <ArrowLeft className="w-6 h-6" />
          </button>
        )}
        <h1 className="text-2xl font-serif tracking-tight text-white select-none">
          {title}
        </h1>
      </div>

      {showEdit && (
        <button
          onClick={onEdit}
          className="p-2 rounded-full hover:bg-white/10 active:scale-95 transition-all text-white/90"
          aria-label="Edit device details"
        >
          <Edit2 className="w-5 h-5" />
        </button>
      )}
    </header>
  );
};
