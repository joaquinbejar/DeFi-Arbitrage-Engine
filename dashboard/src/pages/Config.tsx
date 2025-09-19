import { useState } from 'react';
import { 
  Save, 
  RefreshCw, 
  AlertTriangle, 
  Settings, 
  DollarSign,
  Percent,
  Clock,
  Shield
} from 'lucide-react';
import { toast } from 'sonner';

interface StrategyConfig {
  id: string;
  name: string;
  isActive: boolean;
  minProfitUsd: number;
  minProfitPercent: number;
  maxSlippage: number;
  maxGasPrice: number;
  timeoutMs: number;
  retryAttempts: number;
  dexPreferences: string[];
  tokenWhitelist: string[];
  createdAt: string;
  updatedAt: string;
}

interface RiskLimits {
  maxPositionSize: number;
  maxDailyLoss: number;
  maxConcurrentTrades: number;
  stopLossPercent: number;
  maxDrawdown: number;
  riskScore: number;
}

const Config = () => {
  const [activeTab, setActiveTab] = useState<'strategy' | 'risk' | 'dex' | 'alerts'>('strategy');
  const [isSaving, setSaving] = useState(false);
  
  const [strategyConfig, setStrategyConfig] = useState<StrategyConfig>({
    id: 'default',
    name: 'Default Arbitrage Strategy',
    isActive: true,
    minProfitUsd: 10,
    minProfitPercent: 0.5,
    maxSlippage: 1.0,
    maxGasPrice: 0.01,
    timeoutMs: 30000,
    retryAttempts: 3,
    dexPreferences: ['Raydium', 'Orca', 'Meteora', 'Jupiter'],
    tokenWhitelist: ['SOL', 'USDC', 'USDT', 'RAY', 'ORCA'],
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  });

  const [riskLimits, setRiskLimits] = useState<RiskLimits>({
    maxPositionSize: 10000,
    maxDailyLoss: 1000,
    maxConcurrentTrades: 5,
    stopLossPercent: 5.0,
    maxDrawdown: 2000,
    riskScore: 75
  });

  const [dexSettings, setDexSettings] = useState({
    raydium: { enabled: true, priority: 1, maxSlippage: 1.0 },
    orca: { enabled: true, priority: 2, maxSlippage: 0.8 },
    meteora: { enabled: true, priority: 3, maxSlippage: 1.2 },
    jupiter: { enabled: false, priority: 4, maxSlippage: 1.5 }
  });

  const [alertSettings, setAlertSettings] = useState({
    profitThreshold: 100,
    lossThreshold: 50,
    riskScoreThreshold: 80,
    emailNotifications: true,
    webhookUrl: '',
    slackChannel: ''
  });

  const handleSave = async () => {
    setSaving(true);
    try {
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      toast.success('Configuration saved successfully!');
    } catch (error) {
      toast.error('Failed to save configuration');
    } finally {
      setSaving(false);
    }
  };

  const tabs = [
    { id: 'strategy', name: 'Strategy', icon: Settings },
    { id: 'risk', name: 'Risk Management', icon: Shield },
    { id: 'dex', name: 'DEX Settings', icon: RefreshCw },
    { id: 'alerts', name: 'Alerts', icon: AlertTriangle }
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-white">Configuration</h1>
          <p className="text-gray-400 mt-1">Manage strategy parameters and system settings</p>
        </div>
        <button
          onClick={handleSave}
          disabled={isSaving}
          className="btn-primary flex items-center space-x-2"
        >
          <Save className="h-4 w-4" />
          <span>{isSaving ? 'Saving...' : 'Save Changes'}</span>
        </button>
      </div>

      {/* Tabs */}
      <div className="border-b border-dark-border">
        <nav className="flex space-x-8">
          {tabs.map((tab) => {
            const Icon = tab.icon;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`flex items-center space-x-2 py-2 px-1 border-b-2 font-medium text-sm transition-colors ${
                  activeTab === tab.id
                    ? 'border-primary-green text-primary-green'
                    : 'border-transparent text-gray-400 hover:text-gray-300'
                }`}
              >
                <Icon className="h-4 w-4" />
                <span>{tab.name}</span>
              </button>
            );
          })}
        </nav>
      </div>

      {/* Tab Content */}
      <div className="space-y-6">
        {activeTab === 'strategy' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="card">
              <h3 className="text-lg font-semibold text-white mb-4">Profit Thresholds</h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Minimum Profit (USD)
                  </label>
                  <div className="relative">
                    <DollarSign className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
                    <input
                      type="number"
                      value={strategyConfig.minProfitUsd}
                      onChange={(e) => setStrategyConfig(prev => ({ ...prev, minProfitUsd: Number(e.target.value) }))}
                      className="w-full pl-10 pr-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                      placeholder="10.00"
                    />
                  </div>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Minimum Profit (%)
                  </label>
                  <div className="relative">
                    <Percent className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
                    <input
                      type="number"
                      step="0.1"
                      value={strategyConfig.minProfitPercent}
                      onChange={(e) => setStrategyConfig(prev => ({ ...prev, minProfitPercent: Number(e.target.value) }))}
                      className="w-full pl-10 pr-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                      placeholder="0.5"
                    />
                  </div>
                </div>
              </div>
            </div>

            <div className="card">
              <h3 className="text-lg font-semibold text-white mb-4">Execution Settings</h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Max Slippage (%)
                  </label>
                  <input
                    type="number"
                    step="0.1"
                    value={strategyConfig.maxSlippage}
                    onChange={(e) => setStrategyConfig(prev => ({ ...prev, maxSlippage: Number(e.target.value) }))}
                    className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Timeout (ms)
                  </label>
                  <div className="relative">
                    <Clock className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
                    <input
                      type="number"
                      value={strategyConfig.timeoutMs}
                      onChange={(e) => setStrategyConfig(prev => ({ ...prev, timeoutMs: Number(e.target.value) }))}
                      className="w-full pl-10 pr-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                    />
                  </div>
                </div>
              </div>
            </div>

            <div className="card lg:col-span-2">
              <h3 className="text-lg font-semibold text-white mb-4">Token Whitelist</h3>
              <div className="flex flex-wrap gap-2">
                {strategyConfig.tokenWhitelist.map((token, index) => (
                  <span
                    key={index}
                    className="px-3 py-1 bg-primary-green/10 text-primary-green border border-primary-green/20 rounded-full text-sm"
                  >
                    {token}
                  </span>
                ))}
                <button className="px-3 py-1 border border-dashed border-gray-600 text-gray-400 rounded-full text-sm hover:border-primary-green hover:text-primary-green transition-colors">
                  + Add Token
                </button>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'risk' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="card">
              <h3 className="text-lg font-semibold text-white mb-4">Position Limits</h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Max Position Size (USD)
                  </label>
                  <input
                    type="number"
                    value={riskLimits.maxPositionSize}
                    onChange={(e) => setRiskLimits(prev => ({ ...prev, maxPositionSize: Number(e.target.value) }))}
                    className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Max Concurrent Trades
                  </label>
                  <input
                    type="number"
                    value={riskLimits.maxConcurrentTrades}
                    onChange={(e) => setRiskLimits(prev => ({ ...prev, maxConcurrentTrades: Number(e.target.value) }))}
                    className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                  />
                </div>
              </div>
            </div>

            <div className="card">
              <h3 className="text-lg font-semibold text-white mb-4">Loss Limits</h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Max Daily Loss (USD)
                  </label>
                  <input
                    type="number"
                    value={riskLimits.maxDailyLoss}
                    onChange={(e) => setRiskLimits(prev => ({ ...prev, maxDailyLoss: Number(e.target.value) }))}
                    className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Stop Loss (%)
                  </label>
                  <input
                    type="number"
                    step="0.1"
                    value={riskLimits.stopLossPercent}
                    onChange={(e) => setRiskLimits(prev => ({ ...prev, stopLossPercent: Number(e.target.value) }))}
                    className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                  />
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === 'dex' && (
          <div className="card">
            <h3 className="text-lg font-semibold text-white mb-4">DEX Integrations</h3>
            <div className="space-y-4">
              {Object.entries(dexSettings).map(([dex, settings]) => (
                <div key={dex} className="flex items-center justify-between p-4 bg-dark-border rounded-lg">
                  <div className="flex items-center space-x-4">
                    <div className="flex items-center space-x-2">
                      <input
                        type="checkbox"
                        checked={settings.enabled}
                        onChange={(e) => setDexSettings(prev => ({
                          ...prev,
                          [dex]: { ...settings, enabled: e.target.checked }
                        }))}
                        className="w-4 h-4 text-primary-green bg-dark-bg border-gray-600 rounded focus:ring-primary-green"
                      />
                      <span className="text-white font-medium capitalize">{dex}</span>
                    </div>
                    <div className="flex items-center space-x-2 text-sm text-gray-400">
                      <span>Priority: {settings.priority}</span>
                      <span>•</span>
                      <span>Max Slippage: {settings.maxSlippage}%</span>
                    </div>
                  </div>
                  <div className={`w-2 h-2 rounded-full ${
                    settings.enabled ? 'bg-primary-green' : 'bg-gray-600'
                  }`}></div>
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === 'alerts' && (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="card">
              <h3 className="text-lg font-semibold text-white mb-4">Notification Thresholds</h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Profit Alert Threshold (USD)
                  </label>
                  <input
                    type="number"
                    value={alertSettings.profitThreshold}
                    onChange={(e) => setAlertSettings(prev => ({ ...prev, profitThreshold: Number(e.target.value) }))}
                    className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Loss Alert Threshold (USD)
                  </label>
                  <input
                    type="number"
                    value={alertSettings.lossThreshold}
                    onChange={(e) => setAlertSettings(prev => ({ ...prev, lossThreshold: Number(e.target.value) }))}
                    className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                  />
                </div>
              </div>
            </div>

            <div className="card">
              <h3 className="text-lg font-semibold text-white mb-4">Notification Channels</h3>
              <div className="space-y-4">
                <div className="flex items-center space-x-2">
                  <input
                    type="checkbox"
                    checked={alertSettings.emailNotifications}
                    onChange={(e) => setAlertSettings(prev => ({ ...prev, emailNotifications: e.target.checked }))}
                    className="w-4 h-4 text-primary-green bg-dark-bg border-gray-600 rounded focus:ring-primary-green"
                  />
                  <label className="text-gray-300">Email Notifications</label>
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-300 mb-2">
                    Webhook URL
                  </label>
                  <input
                    type="url"
                    value={alertSettings.webhookUrl}
                    onChange={(e) => setAlertSettings(prev => ({ ...prev, webhookUrl: e.target.value }))}
                    className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:ring-2 focus:ring-primary-green focus:border-transparent"
                    placeholder="https://hooks.slack.com/..."
                  />
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default Config;