import { ReactNode } from "react";

interface AppLayoutProps {
  children: ReactNode;
}

export const AppLayout = ({ children }: AppLayoutProps) => {
  return (
    <div className="min-h-screen bg-black text-white flex flex-col max-w-md mx-auto relative shadow-2xl">
      {children}
    </div>
  );
};
