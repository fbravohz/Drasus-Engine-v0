import "./styles.css";
import { Dashboard } from "./features/dashboard/Dashboard";
import { LocaleProvider } from "./localization/LocaleProvider";
import { WindowTitlebar } from "./components/WindowTitlebar";

function App() {
  return (
    <LocaleProvider>
      <div className="desktop-window">
        <WindowTitlebar />
        <Dashboard />
      </div>
    </LocaleProvider>
  );
}

export default App;
