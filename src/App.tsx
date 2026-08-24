import { AppLayout } from "@/components/layout/AppLayout";
import { DashboardPage } from "@/pages/DashboardPage";
import { EqualizerPage } from "@/pages/EqualizerPage";
import { useDeviceStore } from "@/store/useDeviceStore";

export default function App() {
  const { activeTab } = useDeviceStore();

  return (
    <AppLayout>
      {activeTab === "dashboard" && <DashboardPage />}
      {activeTab === "equalizer" && <EqualizerPage />}
    </AppLayout>
  );
}
