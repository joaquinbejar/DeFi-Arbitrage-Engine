import { useState, useEffect } from 'react';
import { 
  TrendingUp, 
  TrendingDown, 
  DollarSign, 
  Activity, 
  AlertTriangle,
  CheckCircle,
  Clock,
  Zap
} from 'lucide-react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';

interface ArbitrageOpportunity {
  id: string;
  tokenPair: string;
  buyDex: string;
  sellDex: string;
  buyPrice: number;
  sellPrice: number;
  profitUsd: number;
  profitPercent: number;
  volume: number;
  confidence: number;
  estimatedGas: number;
  maxSlippage: number;
  detectedAt: string;
  expiresAt: string;
  status: string;
  hops: any[];
}

interface SystemMetrics {
  totalProfit: number;
  totalTrades: number;
  successRate: number;
  activeOpportunities: number;
  avgExecutionTime: number;
  riskScore: number;
  timestamp: string;
}

const Dashboard = () => {
  const [metrics, setMetrics] = useState<SystemMetrics>({
    totalProfit: 0,
    totalTrades: 0,
    successRate: 0,
    activeOpportunities: 0,
    avgExecutionTime: 0,
    riskScore: 0,
    timestamp: new Date().toISOString()
  });
  
  const [opportunities, setOpportunities] = useState<ArbitrageOpportunity[]>([]);
  const [profitData, setProfitData] = useState<Array<{ time: string; profit: number }>>([]);

  // Mock WebSocket connection for real-time updates
  useEffect(() => {
    // Simulate real-time metrics updates
    const interval = setInterval(() => {
      setMetrics(prev => ({
        ...prev,
        totalProfit: prev.totalProfit + Math.random() * 10,
        totalTrades: prev.totalTrades + Math.floor(Math.random() * 3),
        successRate: 85 + Math.random() * 10,
        activeOpportunities: Math.floor(Math.random() * 15) + 5,
        avgExecutionTime: 150 + Math.random() * 100,
        riskScore: Math.random() * 100,
        timestamp: new Date().toISOString()
      }));
      
      // Update profit chart data
      setProfitData(prev => {
        const newData = [...prev, {
          time: new Date().toLocaleTimeString(),
          profit: Math.random() * 50 + 20
        }].slice(-20); // Keep last 20 points
        return newData;
      });
    }, 3000);

    // Mock opportunities
    const mockOpportunities: ArbitrageOpportunity[] = [
      {
        id: '1',
        tokenPair: 'SOL/USDC',
        buyDex: 'Raydium',
        sellDex: 'Orca',
        buyPrice: 98.45,
        sellPrice: 99.12,
        profitUsd: 67.5,
        profitPercent: 0.68,
        volume: 10000,
        confidence: 0.92,
        estimatedGas: 0.002,
        maxSlippage: 0.5,
        detectedAt: new Date().toISOString(),
        expiresAt: new Date(Date.now() + 30000).toISOString(),
        status: 'detected',
        hops: []
      },
      {
        id: '2',
        tokenPair: 'USDT/SOL',
        buyDex: 'Meteora',
        sellDex: 'Jupiter',
        buyPrice: 0.01015,
        sellPrice: 0.01022,
        profitUsd: 45.2,
        profitPercent: 0.69,
        volume: 8500,
        confidence: 0.88,
        estimatedGas: 0.0018,
        maxSlippage: 0.3,
        detectedAt: new Date().toISOString(),
        expiresAt: new Date(Date.now() + 25000).toISOString(),
        status: 'detected',
        hops: []
      }
    ];
    
    setOpportunities(mockOpportunities);

    return () => clearInterval(interval);
  }, []);

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'detected': return 'text-primary-blue';
      case 'executing': return 'text-yellow-400';
      case 'completed': return 'text-primary-green';
      case 'failed': return 'text-primary-red';
      default: return 'text-gray-400';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'detected': return <Activity className="h-4 w-4" />;
      case 'executing': return <Clock className="h-4 w-4" />;
      case 'completed': return <CheckCircle className="h-4 w-4" />;
      case 'failed': return <AlertTriangle className="h-4 w-4" />;
      default: return <Activity className="h-4 w-4" />;
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-white">Dashboard</h1>
          <p className="text-gray-400 mt-1">Real-time arbitrage monitoring and system metrics</p>
        </div>
        <div className="flex items-center space-x-2 text-sm text-gray-400">
          <div className="w-2 h-2 bg-primary-green rounded-full animate-pulse"></div>
          <span>Live</span>
        </div>
      </div>

      {/* Metrics Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Total Profit</p>
              <p className="metric-value">${metrics.totalProfit.toFixed(2)}</p>
            </div>
            <DollarSign className="h-8 w-8 text-primary-green" />
          </div>
          <div className="flex items-center mt-2 text-sm">
            <TrendingUp className="h-4 w-4 text-primary-green mr-1" />
            <span className="text-primary-green">+12.5%</span>
            <span className="text-gray-400 ml-1">vs last hour</span>
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Total Trades</p>
              <p className="metric-value">{metrics.totalTrades}</p>
            </div>
            <Activity className="h-8 w-8 text-primary-blue" />
          </div>
          <div className="flex items-center mt-2 text-sm">
            <TrendingUp className="h-4 w-4 text-primary-green mr-1" />
            <span className="text-primary-green">+8.2%</span>
            <span className="text-gray-400 ml-1">vs last hour</span>
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Success Rate</p>
              <p className="metric-value">{metrics.successRate.toFixed(1)}%</p>
            </div>
            <CheckCircle className="h-8 w-8 text-primary-green" />
          </div>
          <div className="flex items-center mt-2 text-sm">
            <TrendingDown className="h-4 w-4 text-primary-red mr-1" />
            <span className="text-primary-red">-2.1%</span>
            <span className="text-gray-400 ml-1">vs last hour</span>
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Active Opportunities</p>
              <p className="metric-value">{metrics.activeOpportunities}</p>
            </div>
            <Zap className="h-8 w-8 text-yellow-400" />
          </div>
          <div className="flex items-center mt-2 text-sm">
            <Activity className="h-4 w-4 text-primary-blue mr-1" />
            <span className="text-gray-400">Real-time</span>
          </div>
        </div>
      </div>

      {/* Charts and Opportunities */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Profit Chart */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Profit Trend</h3>
            <div className="text-sm text-gray-400">Last 20 updates</div>
          </div>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={profitData}>
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
                  dataKey="profit" 
                  stroke="#10B981" 
                  strokeWidth={2}
                  dot={false}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Active Opportunities */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Active Opportunities</h3>
            <div className="text-sm text-gray-400">{opportunities.length} detected</div>
          </div>
          <div className="space-y-3 max-h-64 overflow-y-auto">
            {opportunities.map((opp) => (
              <div key={opp.id} className="p-3 bg-dark-border rounded-lg">
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center space-x-2">
                    <span className={getStatusColor(opp.status)}>
                      {getStatusIcon(opp.status)}
                    </span>
                    <span className="font-medium text-white">{opp.tokenPair}</span>
                  </div>
                  <span className="text-primary-green font-semibold">
                    +${opp.profitUsd.toFixed(2)}
                  </span>
                </div>
                <div className="flex justify-between text-sm text-gray-400">
                  <span>{opp.buyDex} → {opp.sellDex}</span>
                  <span>{opp.profitPercent.toFixed(2)}%</span>
                </div>
                <div className="flex justify-between text-xs text-gray-500 mt-1">
                  <span>Confidence: {(opp.confidence * 100).toFixed(0)}%</span>
                  <span>Vol: ${opp.volume.toLocaleString()}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* System Status */}
      <div className="card">
        <h3 className="text-lg font-semibold text-white mb-4">System Status</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="flex items-center justify-between p-3 bg-dark-border rounded-lg">
            <span className="text-gray-300">Execution Engine</span>
            <div className="flex items-center space-x-2">
              <div className="w-2 h-2 bg-primary-green rounded-full"></div>
              <span className="text-primary-green text-sm">Online</span>
            </div>
          </div>
          <div className="flex items-center justify-between p-3 bg-dark-border rounded-lg">
            <span className="text-gray-300">Risk Manager</span>
            <div className="flex items-center space-x-2">
              <div className="w-2 h-2 bg-primary-green rounded-full"></div>
              <span className="text-primary-green text-sm">Active</span>
            </div>
          </div>
          <div className="flex items-center justify-between p-3 bg-dark-border rounded-lg">
            <span className="text-gray-300">DEX Connections</span>
            <div className="flex items-center space-x-2">
              <div className="w-2 h-2 bg-yellow-400 rounded-full"></div>
              <span className="text-yellow-400 text-sm">3/4 Online</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Dashboard;