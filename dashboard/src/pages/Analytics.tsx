import { useState, useEffect } from 'react';
import { 
  TrendingUp, 
  BarChart3, 
  PieChart, 
  Calendar,
  Download,
  RefreshCw
} from 'lucide-react';
import { 
  AreaChart, 
  Area, 
  BarChart, 
  Bar, 
  PieChart as RechartsPieChart, 
  Pie, 
  Cell, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  ResponsiveContainer 
} from 'recharts';

const Analytics = () => {
  const [timeRange, setTimeRange] = useState<'1h' | '24h' | '7d' | '30d'>('24h');
  const [profitData, setProfitData] = useState<Array<{ time: string; profit: number; volume: number }>>([]);
  const [dexData, setDexData] = useState<Array<{ name: string; trades: number; profit: number; color: string }>>([]);
  const [tokenData, setTokenData] = useState<Array<{ pair: string; trades: number; profit: number; successRate: number }>>([]);
  const [performanceMetrics, setPerformanceMetrics] = useState({
    totalProfit: 0,
    totalVolume: 0,
    avgProfitPerTrade: 0,
    bestTrade: 0,
    worstTrade: 0,
    sharpeRatio: 0,
    maxDrawdown: 0,
    winRate: 0
  });

  useEffect(() => {
    // Generate mock analytics data based on time range
    const generateData = () => {
      const now = new Date();
      const dataPoints = timeRange === '1h' ? 12 : timeRange === '24h' ? 24 : timeRange === '7d' ? 7 : 30;
      const interval = timeRange === '1h' ? 5 * 60 * 1000 : timeRange === '24h' ? 60 * 60 * 1000 : 24 * 60 * 60 * 1000;

      const profit = [];
      for (let i = dataPoints - 1; i >= 0; i--) {
        const time = new Date(now.getTime() - i * interval);
        profit.push({
          time: timeRange === '1h' || timeRange === '24h' 
            ? time.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
            : time.toLocaleDateString([], { month: 'short', day: 'numeric' }),
          profit: Math.random() * 100 + 20,
          volume: Math.random() * 50000 + 10000
        });
      }
      setProfitData(profit);

      // DEX performance data
      const dexPerformance = [
        { name: 'Raydium', trades: 145, profit: 2340.50, color: '#8B5CF6' },
        { name: 'Orca', trades: 132, profit: 1890.25, color: '#10B981' },
        { name: 'Meteora', trades: 98, profit: 1456.75, color: '#F59E0B' },
        { name: 'Jupiter', trades: 76, profit: 987.30, color: '#3B82F6' }
      ];
      setDexData(dexPerformance);

      // Token pair performance
      const tokenPerformance = [
        { pair: 'SOL/USDC', trades: 89, profit: 1234.56, successRate: 87.5 },
        { pair: 'USDT/SOL', trades: 76, profit: 987.43, successRate: 82.1 },
        { pair: 'RAY/USDC', trades: 54, profit: 678.90, successRate: 79.6 },
        { pair: 'ORCA/SOL', trades: 43, profit: 543.21, successRate: 74.4 },
        { pair: 'USDC/USDT', trades: 32, profit: 234.67, successRate: 68.8 }
      ];
      setTokenData(tokenPerformance);

      // Performance metrics
      const totalProfit = profit.reduce((sum, p) => sum + p.profit, 0);
      const totalVolume = profit.reduce((sum, p) => sum + p.volume, 0);
      const totalTrades = dexPerformance.reduce((sum, d) => sum + d.trades, 0);
      
      setPerformanceMetrics({
        totalProfit,
        totalVolume,
        avgProfitPerTrade: totalProfit / totalTrades,
        bestTrade: Math.max(...profit.map(p => p.profit)),
        worstTrade: Math.min(...profit.map(p => p.profit)) * -0.3, // Simulate some losses
        sharpeRatio: 2.34,
        maxDrawdown: 156.78,
        winRate: 84.2
      });
    };

    generateData();
  }, [timeRange]);



  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-white">Analytics</h1>
          <p className="text-gray-400 mt-1">Performance analysis and trading insights</p>
        </div>
        <div className="flex items-center space-x-4">
          {/* Time Range Selector */}
          <div className="flex bg-dark-border rounded-lg p-1">
            {(['1h', '24h', '7d', '30d'] as const).map((range) => (
              <button
                key={range}
                onClick={() => setTimeRange(range)}
                className={`px-3 py-1 rounded text-sm font-medium transition-colors ${
                  timeRange === range
                    ? 'bg-primary-green text-white'
                    : 'text-gray-400 hover:text-white'
                }`}
              >
                {range}
              </button>
            ))}
          </div>
          <button className="flex items-center space-x-2 px-4 py-2 bg-dark-border text-gray-300 rounded-lg hover:bg-gray-600 transition-colors">
            <Download className="h-4 w-4" />
            <span>Export</span>
          </button>
        </div>
      </div>

      {/* Performance Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Total Profit</p>
              <p className="metric-value">${performanceMetrics.totalProfit.toFixed(2)}</p>
            </div>
            <TrendingUp className="h-8 w-8 text-primary-green" />
          </div>
          <div className="flex items-center mt-2 text-sm">
            <span className="text-primary-green">↗ +15.3%</span>
            <span className="text-gray-400 ml-1">vs previous period</span>
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Win Rate</p>
              <p className="metric-value">{performanceMetrics.winRate.toFixed(1)}%</p>
            </div>
            <BarChart3 className="h-8 w-8 text-primary-blue" />
          </div>
          <div className="flex items-center mt-2 text-sm">
            <span className="text-primary-green">↗ +2.1%</span>
            <span className="text-gray-400 ml-1">vs previous period</span>
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Sharpe Ratio</p>
              <p className="metric-value">{performanceMetrics.sharpeRatio.toFixed(2)}</p>
            </div>
            <PieChart className="h-8 w-8 text-yellow-400" />
          </div>
          <div className="flex items-center mt-2 text-sm">
            <span className="text-primary-green">↗ +0.15</span>
            <span className="text-gray-400 ml-1">vs previous period</span>
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Max Drawdown</p>
              <p className="metric-value">${performanceMetrics.maxDrawdown.toFixed(2)}</p>
            </div>
            <Calendar className="h-8 w-8 text-primary-red" />
          </div>
          <div className="flex items-center mt-2 text-sm">
            <span className="text-primary-red">↘ -5.2%</span>
            <span className="text-gray-400 ml-1">vs previous period</span>
          </div>
        </div>
      </div>

      {/* Charts Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Profit Trend */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Profit Trend</h3>
            <RefreshCw className="h-4 w-4 text-gray-400" />
          </div>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={profitData}>
                <defs>
                  <linearGradient id="profitGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#10B981" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#10B981" stopOpacity={0}/>
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
                  dataKey="profit" 
                  stroke="#10B981" 
                  fillOpacity={1} 
                  fill="url(#profitGradient)"
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Volume Analysis */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">Trading Volume</h3>
          </div>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={profitData}>
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
                <Bar dataKey="volume" fill="#3B82F6" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* DEX Performance */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">DEX Performance</h3>
          </div>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <RechartsPieChart>
                <Pie
                  data={dexData}
                  cx="50%"
                  cy="50%"
                  outerRadius={80}
                  fill="#8884d8"
                  dataKey="profit"
                  label={(props: { name: string; percent: number }) => `${props.name} ${(props.percent * 100).toFixed(0)}%`}
                >
                  {dexData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color} />
                  ))}
                </Pie>
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#1F2937', 
                    border: '1px solid #374151',
                    borderRadius: '8px'
                  }}
                />
              </RechartsPieChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Profit Distribution */}
        <div className="card">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold text-white">DEX Comparison</h3>
          </div>
          <div className="h-80">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={dexData} layout="horizontal">
                <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                <XAxis type="number" stroke="#9CA3AF" fontSize={12} />
                <YAxis dataKey="name" type="category" stroke="#9CA3AF" fontSize={12} />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#1F2937', 
                    border: '1px solid #374151',
                    borderRadius: '8px'
                  }}
                />
                <Bar dataKey="profit" fill="#10B981" radius={[0, 4, 4, 0]} />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Token Pair Performance Table */}
      <div className="card">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-white">Token Pair Performance</h3>
          <span className="text-sm text-gray-400">Top performing pairs</span>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-dark-border">
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Pair</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Trades</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Total Profit</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Avg Profit</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Success Rate</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Performance</th>
              </tr>
            </thead>
            <tbody>
              {tokenData.map((token, index) => (
                <tr key={token.pair} className="border-b border-dark-border hover:bg-dark-border/50">
                  <td className="py-3 px-4">
                    <span className="text-white font-medium">{token.pair}</span>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-gray-300">{token.trades}</span>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-primary-green font-medium">
                      ${token.profit.toFixed(2)}
                    </span>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-gray-300">
                      ${(token.profit / token.trades).toFixed(2)}
                    </span>
                  </td>
                  <td className="py-3 px-4">
                    <div className="flex items-center space-x-2">
                      <span className={`text-sm font-medium ${
                        token.successRate >= 80 ? 'text-primary-green' :
                        token.successRate >= 70 ? 'text-yellow-400' : 'text-primary-red'
                      }`}>
                        {token.successRate.toFixed(1)}%
                      </span>
                    </div>
                  </td>
                  <td className="py-3 px-4">
                    <div className="flex items-center space-x-2">
                      <div className="w-16 bg-dark-border rounded-full h-2">
                        <div 
                          className={`h-2 rounded-full ${
                            index === 0 ? 'bg-primary-green' :
                            index === 1 ? 'bg-primary-blue' :
                            index === 2 ? 'bg-yellow-400' : 'bg-gray-500'
                          }`}
                          style={{ width: `${Math.min(token.successRate, 100)}%` }}
                        ></div>
                      </div>
                      <span className="text-xs text-gray-400">
                        #{index + 1}
                      </span>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Additional Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="card">
          <h4 className="text-md font-semibold text-white mb-3">Best Trade</h4>
          <div className="text-2xl font-bold text-primary-green mb-1">
            +${performanceMetrics.bestTrade.toFixed(2)}
          </div>
          <p className="text-sm text-gray-400">SOL/USDC on Raydium → Orca</p>
        </div>

        <div className="card">
          <h4 className="text-md font-semibold text-white mb-3">Worst Trade</h4>
          <div className="text-2xl font-bold text-primary-red mb-1">
            ${performanceMetrics.worstTrade.toFixed(2)}
          </div>
          <p className="text-sm text-gray-400">RAY/USDC execution failed</p>
        </div>

        <div className="card">
          <h4 className="text-md font-semibold text-white mb-3">Avg Profit/Trade</h4>
          <div className="text-2xl font-bold text-primary-blue mb-1">
            ${performanceMetrics.avgProfitPerTrade.toFixed(2)}
          </div>
          <p className="text-sm text-gray-400">Across all executed trades</p>
        </div>
      </div>
    </div>
  );
};

export default Analytics;