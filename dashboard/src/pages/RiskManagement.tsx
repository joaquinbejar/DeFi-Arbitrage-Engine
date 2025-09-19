import { useState, useEffect } from 'react';
import { 
  Shield, 
  AlertTriangle, 
  TrendingDown, 
  DollarSign,
  Clock,
  Activity,
  Settings,
  Pause,
  Play,
  StopCircle
} from 'lucide-react';
import { 
  LineChart, 
  Line, 
  AreaChart, 
  Area, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  ResponsiveContainer 
} from 'recharts';
import { toast } from 'sonner';

interface RiskAlert {
  id: string;
  type: 'high' | 'medium' | 'low';
  message: string;
  timestamp: Date;
  resolved: boolean;
}

interface RiskMetric {
  name: string;
  current: number;
  limit: number;
  status: 'safe' | 'warning' | 'danger';
  unit: string;
}

const RiskManagement = () => {
  const [systemStatus, setSystemStatus] = useState<'active' | 'paused' | 'emergency'>('active');
  const [riskScore] = useState(23);
  const [alerts, setAlerts] = useState<RiskAlert[]>([]);
  const [riskMetrics, setRiskMetrics] = useState<RiskMetric[]>([]);
  const [drawdownData, setDrawdownData] = useState<Array<{ time: string; drawdown: number; equity: number }>>([]);
  const [riskLimits, setRiskLimits] = useState({
    maxPositionSize: 10000,
    maxDailyLoss: 500,
    maxDrawdown: 1000,
    maxSlippage: 2.5,
    minLiquidity: 50000,
    maxGasPrice: 100
  });
  const [emergencySettings, setEmergencySettings] = useState({
    autoStopLoss: true,
    emergencyWithdraw: false,
    pauseOnHighRisk: true,
    notifyOnAlert: true
  });

  useEffect(() => {
    // Generate mock risk data
    const generateRiskData = () => {
      // Risk alerts
      const mockAlerts: RiskAlert[] = [
        {
          id: '1',
          type: 'high',
          message: 'Daily loss limit approaching (85% of limit)',
          timestamp: new Date(Date.now() - 5 * 60 * 1000),
          resolved: false
        },
        {
          id: '2',
          type: 'medium',
          message: 'High slippage detected on Raydium SOL/USDC pool',
          timestamp: new Date(Date.now() - 15 * 60 * 1000),
          resolved: false
        },
        {
          id: '3',
          type: 'low',
          message: 'Gas price spike detected (120 gwei)',
          timestamp: new Date(Date.now() - 30 * 60 * 1000),
          resolved: true
        },
        {
          id: '4',
          type: 'medium',
          message: 'Liquidity below threshold on ORCA/USDT pair',
          timestamp: new Date(Date.now() - 45 * 60 * 1000),
          resolved: false
        }
      ];
      setAlerts(mockAlerts);

      // Risk metrics
      const mockMetrics: RiskMetric[] = [
        {
          name: 'Position Size',
          current: 8500,
          limit: riskLimits.maxPositionSize,
          status: 'warning',
          unit: '$'
        },
        {
          name: 'Daily Loss',
          current: 425,
          limit: riskLimits.maxDailyLoss,
          status: 'danger',
          unit: '$'
        },
        {
          name: 'Max Drawdown',
          current: 750,
          limit: riskLimits.maxDrawdown,
          status: 'warning',
          unit: '$'
        },
        {
          name: 'Avg Slippage',
          current: 1.8,
          limit: riskLimits.maxSlippage,
          status: 'safe',
          unit: '%'
        },
        {
          name: 'Min Liquidity',
          current: 75000,
          limit: riskLimits.minLiquidity,
          status: 'safe',
          unit: '$'
        },
        {
          name: 'Gas Price',
          current: 85,
          limit: riskLimits.maxGasPrice,
          status: 'safe',
          unit: 'gwei'
        }
      ];
      setRiskMetrics(mockMetrics);

      // Drawdown chart data
      const drawdown = [];
      const now = new Date();
      for (let i = 23; i >= 0; i--) {
        const time = new Date(now.getTime() - i * 60 * 60 * 1000);
        drawdown.push({
          time: time.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          drawdown: Math.random() * -800 - 100,
          equity: 10000 + Math.random() * 2000 - 1000
        });
      }
      setDrawdownData(drawdown);
    };

    generateRiskData();
    const interval = setInterval(generateRiskData, 30000); // Update every 30 seconds
    return () => clearInterval(interval);
  }, [riskLimits]);

  const handleSystemControl = (action: 'pause' | 'resume' | 'emergency') => {
    switch (action) {
      case 'pause':
        setSystemStatus('paused');
        toast.success('System paused successfully');
        break;
      case 'resume':
        setSystemStatus('active');
        toast.success('System resumed successfully');
        break;
      case 'emergency':
        setSystemStatus('emergency');
        toast.error('Emergency stop activated!');
        break;
    }
  };

  const handleResolveAlert = (alertId: string) => {
    setAlerts(prev => prev.map(alert => 
      alert.id === alertId ? { ...alert, resolved: true } : alert
    ));
    toast.success('Alert resolved');
  };

  const handleUpdateLimits = () => {
    toast.success('Risk limits updated successfully');
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'text-primary-green';
      case 'paused': return 'text-yellow-400';
      case 'emergency': return 'text-primary-red';
      default: return 'text-gray-400';
    }
  };

  const getStatusIcon = () => {
    switch (systemStatus) {
      case 'active': return <Activity className="h-5 w-5" />;
      case 'paused': return <Pause className="h-5 w-5" />;
      case 'emergency': return <StopCircle className="h-5 w-5" />;
    }
  };

  const getRiskScoreColor = (score: number) => {
    if (score <= 30) return 'text-primary-green';
    if (score <= 60) return 'text-yellow-400';
    return 'text-primary-red';
  };

  const getMetricStatusColor = (status: string) => {
    switch (status) {
      case 'safe': return 'text-primary-green';
      case 'warning': return 'text-yellow-400';
      case 'danger': return 'text-primary-red';
      default: return 'text-gray-400';
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-white">Risk Management</h1>
          <p className="text-gray-400 mt-1">Monitor and control system risk exposure</p>
        </div>
        <div className="flex items-center space-x-4">
          <div className={`flex items-center space-x-2 px-3 py-2 rounded-lg bg-dark-border ${getStatusColor(systemStatus)}`}>
            {getStatusIcon()}
            <span className="font-medium capitalize">{systemStatus}</span>
          </div>
        </div>
      </div>

      {/* System Controls */}
      <div className="card">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-white">System Controls</h3>
          <div className={`text-2xl font-bold ${getRiskScoreColor(riskScore)}`}>
            Risk Score: {riskScore}/100
          </div>
        </div>
        <div className="flex items-center space-x-4">
          <button
            onClick={() => handleSystemControl('pause')}
            disabled={systemStatus === 'paused'}
            className="flex items-center space-x-2 px-4 py-2 bg-yellow-600 hover:bg-yellow-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
          >
            <Pause className="h-4 w-4" />
            <span>Pause System</span>
          </button>
          <button
            onClick={() => handleSystemControl('resume')}
            disabled={systemStatus === 'active'}
            className="flex items-center space-x-2 px-4 py-2 bg-primary-green hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white rounded-lg transition-colors"
          >
            <Play className="h-4 w-4" />
            <span>Resume System</span>
          </button>
          <button
            onClick={() => handleSystemControl('emergency')}
            className="flex items-center space-x-2 px-4 py-2 bg-primary-red hover:bg-red-700 text-white rounded-lg transition-colors"
          >
            <StopCircle className="h-4 w-4" />
            <span>Emergency Stop</span>
          </button>
        </div>
      </div>

      {/* Risk Metrics Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {riskMetrics.map((metric, index) => (
          <div key={index} className="metric-card">
            <div className="flex items-center justify-between mb-2">
              <p className="metric-label">{metric.name}</p>
              <Shield className={`h-5 w-5 ${getMetricStatusColor(metric.status)}`} />
            </div>
            <div className="flex items-baseline space-x-2">
              <span className={`text-2xl font-bold ${getMetricStatusColor(metric.status)}`}>
                {metric.unit === '$' ? '$' : ''}{metric.current.toLocaleString()}{metric.unit !== '$' ? metric.unit : ''}
              </span>
              <span className="text-sm text-gray-400">
                / {metric.unit === '$' ? '$' : ''}{metric.limit.toLocaleString()}{metric.unit !== '$' ? metric.unit : ''}
              </span>
            </div>
            <div className="mt-3">
              <div className="w-full bg-dark-border rounded-full h-2">
                <div 
                  className={`h-2 rounded-full transition-all duration-300 ${
                    metric.status === 'safe' ? 'bg-primary-green' :
                    metric.status === 'warning' ? 'bg-yellow-400' : 'bg-primary-red'
                  }`}
                  style={{ width: `${Math.min((metric.current / metric.limit) * 100, 100)}%` }}
                ></div>
              </div>
              <div className="flex justify-between text-xs text-gray-400 mt-1">
                <span>0</span>
                <span>{((metric.current / metric.limit) * 100).toFixed(1)}%</span>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Drawdown Chart */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Drawdown Analysis</h3>
            <TrendingDown className="h-5 w-5 text-primary-red" />
          </div>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={drawdownData}>
                <defs>
                  <linearGradient id="drawdownGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#EF4444" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#EF4444" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                <XAxis dataKey="time" stroke="#9CA3AF" fontSize={12} />
                <YAxis stroke="#9CA3AF" fontSize={12} />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#1F2937', 
                    border: '1px solid #374151',
                    borderRadius: '8px'
                  }}
                />
                <Area 
                  type="monotone" 
                  dataKey="drawdown" 
                  stroke="#EF4444" 
                  fillOpacity={1} 
                  fill="url(#drawdownGradient)"
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Equity Curve */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Equity Curve</h3>
            <DollarSign className="h-5 w-5 text-primary-green" />
          </div>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={drawdownData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                <XAxis dataKey="time" stroke="#9CA3AF" fontSize={12} />
                <YAxis stroke="#9CA3AF" fontSize={12} />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#1F2937', 
                    border: '1px solid #374151',
                    borderRadius: '8px'
                  }}
                />
                <Line 
                  type="monotone" 
                  dataKey="equity" 
                  stroke="#10B981" 
                  strokeWidth={2}
                  dot={false}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Risk Alerts */}
      <div className="card">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-white">Risk Alerts</h3>
          <span className="text-sm text-gray-400">
            {alerts.filter(a => !a.resolved).length} active alerts
          </span>
        </div>
        <div className="space-y-3">
          {alerts.map((alert) => (
            <div 
              key={alert.id} 
              className={`flex items-center justify-between p-4 rounded-lg border ${
                alert.resolved 
                  ? 'bg-gray-800/50 border-gray-600' 
                  : alert.type === 'high' 
                    ? 'bg-red-900/20 border-red-500/50' 
                    : alert.type === 'medium'
                      ? 'bg-yellow-900/20 border-yellow-500/50'
                      : 'bg-blue-900/20 border-blue-500/50'
              }`}
            >
              <div className="flex items-center space-x-3">
                <AlertTriangle className={`h-5 w-5 ${
                  alert.resolved 
                    ? 'text-gray-400' 
                    : alert.type === 'high' 
                      ? 'text-primary-red' 
                      : alert.type === 'medium'
                        ? 'text-yellow-400'
                        : 'text-primary-blue'
                }`} />
                <div>
                  <p className={`font-medium ${
                    alert.resolved ? 'text-gray-400' : 'text-white'
                  }`}>
                    {alert.message}
                  </p>
                  <div className="flex items-center space-x-2 text-sm text-gray-400">
                    <Clock className="h-3 w-3" />
                    <span>{alert.timestamp.toLocaleTimeString()}</span>
                    <span className={`px-2 py-1 rounded text-xs ${
                      alert.type === 'high' ? 'bg-red-900/50 text-red-300' :
                      alert.type === 'medium' ? 'bg-yellow-900/50 text-yellow-300' :
                      'bg-blue-900/50 text-blue-300'
                    }`}>
                      {alert.type.toUpperCase()}
                    </span>
                  </div>
                </div>
              </div>
              {!alert.resolved && (
                <button
                  onClick={() => handleResolveAlert(alert.id)}
                  className="px-3 py-1 bg-primary-green hover:bg-green-700 text-white text-sm rounded transition-colors"
                >
                  Resolve
                </button>
              )}
            </div>
          ))}
        </div>
      </div>

      {/* Risk Settings */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Risk Limits */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Risk Limits</h3>
            <Settings className="h-5 w-5 text-gray-400" />
          </div>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Max Position Size ($)
              </label>
              <input
                type="number"
                value={riskLimits.maxPositionSize}
                onChange={(e) => setRiskLimits(prev => ({ ...prev, maxPositionSize: Number(e.target.value) }))}
                className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:outline-none focus:border-primary-green"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Max Daily Loss ($)
              </label>
              <input
                type="number"
                value={riskLimits.maxDailyLoss}
                onChange={(e) => setRiskLimits(prev => ({ ...prev, maxDailyLoss: Number(e.target.value) }))}
                className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:outline-none focus:border-primary-green"
              />
            </div>
            <div>
              <label className="block text-sm font-medium text-gray-300 mb-1">
                Max Drawdown ($)
              </label>
              <input
                type="number"
                value={riskLimits.maxDrawdown}
                onChange={(e) => setRiskLimits(prev => ({ ...prev, maxDrawdown: Number(e.target.value) }))}
                className="w-full px-3 py-2 bg-dark-border border border-gray-600 rounded-lg text-white focus:outline-none focus:border-primary-green"
              />
            </div>
            <button
              onClick={handleUpdateLimits}
              className="w-full btn-primary"
            >
              Update Limits
            </button>
          </div>
        </div>

        {/* Emergency Settings */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Emergency Settings</h3>
            <AlertTriangle className="h-5 w-5 text-primary-red" />
          </div>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-gray-300">Auto Stop Loss</span>
              <button
                onClick={() => setEmergencySettings(prev => ({ ...prev, autoStopLoss: !prev.autoStopLoss }))}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  emergencySettings.autoStopLoss ? 'bg-primary-green' : 'bg-gray-600'
                }`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  emergencySettings.autoStopLoss ? 'translate-x-6' : 'translate-x-1'
                }`} />
              </button>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-gray-300">Emergency Withdraw</span>
              <button
                onClick={() => setEmergencySettings(prev => ({ ...prev, emergencyWithdraw: !prev.emergencyWithdraw }))}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  emergencySettings.emergencyWithdraw ? 'bg-primary-red' : 'bg-gray-600'
                }`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  emergencySettings.emergencyWithdraw ? 'translate-x-6' : 'translate-x-1'
                }`} />
              </button>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-gray-300">Pause on High Risk</span>
              <button
                onClick={() => setEmergencySettings(prev => ({ ...prev, pauseOnHighRisk: !prev.pauseOnHighRisk }))}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  emergencySettings.pauseOnHighRisk ? 'bg-primary-green' : 'bg-gray-600'
                }`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  emergencySettings.pauseOnHighRisk ? 'translate-x-6' : 'translate-x-1'
                }`} />
              </button>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-gray-300">Notify on Alert</span>
              <button
                onClick={() => setEmergencySettings(prev => ({ ...prev, notifyOnAlert: !prev.notifyOnAlert }))}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                  emergencySettings.notifyOnAlert ? 'bg-primary-green' : 'bg-gray-600'
                }`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  emergencySettings.notifyOnAlert ? 'translate-x-6' : 'translate-x-1'
                }`} />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default RiskManagement;