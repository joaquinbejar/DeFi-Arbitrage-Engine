import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Toaster } from 'sonner';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import Config from './pages/Config';
import ExecutionMonitor from './pages/ExecutionMonitor';
import Analytics from './pages/Analytics';
import RiskManagement from './pages/RiskManagement';
import Login from './pages/Login';
import { useAuthStore } from './stores/authStore';

function App() {
  const { isAuthenticated } = useAuthStore();

  if (!isAuthenticated) {
    return (
      <div className="min-h-screen bg-dark-bg">
        <Login />
        <Toaster theme="dark" />
      </div>
    );
  }

  return (
    <Router>
      <div className="min-h-screen bg-dark-bg">
        <Layout>
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/config" element={<Config />} />
            <Route path="/execution" element={<ExecutionMonitor />} />
            <Route path="/analytics" element={<Analytics />} />
            <Route path="/risk" element={<RiskManagement />} />
          </Routes>
        </Layout>
        <Toaster theme="dark" />
      </div>
    </Router>
  );
}

export default App;
