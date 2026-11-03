import "./styles.css";
import { Dashboard } from "./features/dashboard/Dashboard";
import { LocaleProvider } from "./localization/LocaleProvider";

function App() {
  return (
    <LocaleProvider>
      <Dashboard />
    </LocaleProvider>
  );
}

export default App;
