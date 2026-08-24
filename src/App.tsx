import { AppLayout } from "@/components/layout/AppLayout";
import { DashboardPage } from "@/pages/DashboardPage";
import { useDeviceStore } from "@/store/useDeviceStore";

export default function App() {
  const { activeTab } = useDeviceStore();

  return (
    <AppLayout>
      {activeTab === "dashboard" && <DashboardPage />}
      {/* Subpages will be hooked in Tasks 11 & 12 */}
    </AppLayout>
  );
}
