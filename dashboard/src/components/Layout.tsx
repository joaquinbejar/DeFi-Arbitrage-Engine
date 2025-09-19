import { type ReactNode } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { 
  BarChart3, 
  Settings, 
  Activity, 
  TrendingUp, 
  Shield, 
  LogOut,
  Zap
} from 'lucide-react';
import { useAuthStore } from '../stores/authStore';

interface LayoutProps {
  children: ReactNode;
}

const Layout = ({ children }: LayoutProps) => {
  const location = useLocation();
  const { logout, user } = useAuthStore();

  const navigation = [
    { name: 'Dashboard', href: '/', icon: BarChart3 },
    { name: 'Configuration', href: '/config', icon: Settings },
    { name: 'Execution Monitor', href: '/execution', icon: Activity },
    { name: 'Analytics', href: '/analytics', icon: TrendingUp },
    { name: 'Risk Management', href: '/risk', icon: Shield },
  ];

  const isActive = (path: string) => {
    return location.pathname === path;
  };

  return (
    <div className="flex h-screen bg-dark-bg">
      {/* Sidebar */}
      <div className="w-64 bg-dark-card border-r border-dark-border flex flex-col">
        {/* Logo */}
        <div className="p-6 border-b border-dark-border">
          <div className="flex items-center space-x-2">
            <Zap className="h-8 w-8 text-primary-green" />
            <div>
              <h1 className="text-xl font-bold text-white">Arbitrage Engine</h1>
              <p className="text-xs text-gray-400">Solana DeFi</p>
            </div>
          </div>
        </div>

        {/* Navigation */}
        <nav className="flex-1 p-4 space-y-2">
          {navigation.map((item) => {
            const Icon = item.icon;
            return (
              <Link
                key={item.name}
                to={item.href}
                className={`flex items-center space-x-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                  isActive(item.href)
                    ? 'bg-primary-green/10 text-primary-green border border-primary-green/20'
                    : 'text-gray-300 hover:text-white hover:bg-dark-border'
                }`}
              >
                <Icon className="h-5 w-5" />
                <span>{item.name}</span>
              </Link>
            );
          })}
        </nav>

        {/* User Info & Logout */}
        <div className="p-4 border-t border-dark-border">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-primary-green rounded-full flex items-center justify-center">
                <span className="text-xs font-bold text-white">
                  {user?.publicKey.slice(0, 2).toUpperCase()}
                </span>
              </div>
              <div>
                <p className="text-sm font-medium text-white">
                  {user?.publicKey.slice(0, 6)}...{user?.publicKey.slice(-4)}
                </p>
                <p className="text-xs text-gray-400 capitalize">{user?.role}</p>
              </div>
            </div>
            <button
              onClick={logout}
              className="p-2 text-gray-400 hover:text-white hover:bg-dark-border rounded-lg transition-colors"
              title="Logout"
            >
              <LogOut className="h-4 w-4" />
            </button>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        <main className="flex-1 overflow-auto p-6">
          {children}
        </main>
      </div>
    </div>
  );
};

export default Layout;