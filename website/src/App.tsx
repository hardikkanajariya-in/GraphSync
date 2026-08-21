import { BrowserRouter, Navigate, Route, Routes } from "react-router-dom";
import { DocsLayout } from "./layouts/DocsLayout";
import { LandingPage } from "./pages/LandingPage";
import { ApiReferencePage } from "./pages/docs/ApiReferencePage";
import { DesktopClientPage } from "./pages/docs/DesktopClientPage";
import { GettingStartedPage } from "./pages/docs/GettingStartedPage";
import { HowItWorksPage } from "./pages/docs/HowItWorksPage";
import { DocsIntroPage } from "./pages/docs/IntroPage";
import { SecurityPage } from "./pages/docs/SecurityPage";
import { SyncProtocolPage } from "./pages/docs/SyncProtocolPage";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route path="/docs" element={<DocsLayout />}>
          <Route index element={<DocsIntroPage />} />
          <Route path="how-it-works" element={<HowItWorksPage />} />
          <Route path="getting-started" element={<GettingStartedPage />} />
          <Route path="desktop-client" element={<DesktopClientPage />} />
          <Route path="sync-protocol" element={<SyncProtocolPage />} />
          <Route path="api" element={<ApiReferencePage />} />
          <Route path="security" element={<SecurityPage />} />
        </Route>
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </BrowserRouter>
  );
}
