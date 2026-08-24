import { AppLayout } from "@/components/layout/AppLayout";
import { DashboardPage } from "@/pages/DashboardPage";
import { DeviceSettingsPage } from "@/pages/DeviceSettingsPage";
import { EqualizerPage } from "@/pages/EqualizerPage";
import { GesturesPage } from "@/pages/GesturesPage";
import { SystemSettingsPage } from "@/pages/SystemSettingsPage";
import { useDeviceStore } from "@/store/useDeviceStore";

export default function App() {
  const { activeTab } = useDeviceStore();

  return (
    <AppLayout>
      {activeTab === "dashboard" && <DashboardPage />}
      {activeTab === "equalizer" && <EqualizerPage />}
      {activeTab === "gestures" && <GesturesPage />}
      {activeTab === "device_settings" && <DeviceSettingsPage />}
      {activeTab === "system_settings" && <SystemSettingsPage />}
    </AppLayout>
  );
}
