import { useState, useEffect } from 'react';
import { 
  Play, 
  Pause, 
  CheckCircle, 
  XCircle, 
  Clock, 
  Activity,
  TrendingUp,
  AlertTriangle,
  ExternalLink,
  Filter
} from 'lucide-react';

interface ExecutionResult {
  id: string;
  opportunityId: string;
  status: 'pending' | 'executing' | 'completed' | 'failed';
  tokenPair: string;
  buyDex: string;
  sellDex: string;
  executedAmount: number;
  actualProfit: number;
  actualProfitPercent: number;
  gasUsed: number;
  executionTime: number;
  slippage: number;
  txHash: string | null;
  startedAt: string;
  completedAt: string | null;
  error: string | null;
}

const ExecutionMonitor = () => {
  const [isAutoRefresh, setIsAutoRefresh] = useState(true);
  const [filter, setFilter] = useState<'all' | 'executing' | 'completed' | 'failed'>('all');
  const [executions, setExecutions] = useState<ExecutionResult[]>([]);
  const [stats, setStats] = useState({
    total: 0,
    executing: 0,
    completed: 0,
    failed: 0,
    totalProfit: 0,
    avgExecutionTime: 0
  });

  // Mock execution data
  useEffect(() => {
    const mockExecutions: ExecutionResult[] = [
      {
        id: 'exec_001',
        opportunityId: 'opp_001',
        status: 'completed',
        tokenPair: 'SOL/USDC',
        buyDex: 'Raydium',
        sellDex: 'Orca',
        executedAmount: 1000,
        actualProfit: 45.67,
        actualProfitPercent: 4.57,
        gasUsed: 0.0023,
        executionTime: 1850,
        slippage: 0.12,
        txHash: '5KJp9X2vN8qR4mF7wE3tY6uH9sL2cV1bA8nM4xZ7pQ9rT3kW6jS',
        startedAt: new Date(Date.now() - 120000).toISOString(),
        completedAt: new Date(Date.now() - 118000).toISOString(),
        error: null
      },
      {
        id: 'exec_002',
        opportunityId: 'opp_002',
        status: 'executing',
        tokenPair: 'USDT/SOL',
        buyDex: 'Meteora',
        sellDex: 'Jupiter',
        executedAmount: 750,
        actualProfit: 0,
        actualProfitPercent: 0,
        gasUsed: 0,
        executionTime: 0,
        slippage: 0,
        txHash: null,
        startedAt: new Date(Date.now() - 15000).toISOString(),
        completedAt: null,
        error: null
      },
      {
        id: 'exec_003',
        opportunityId: 'opp_003',
        status: 'failed',
        tokenPair: 'RAY/USDC',
        buyDex: 'Orca',
        sellDex: 'Raydium',
        executedAmount: 0,
        actualProfit: -2.34,
        actualProfitPercent: -0.23,
        gasUsed: 0.0018,
        executionTime: 3200,
        slippage: 0,
        txHash: null,
        startedAt: new Date(Date.now() - 300000).toISOString(),
        completedAt: new Date(Date.now() - 297000).toISOString(),
        error: 'Insufficient liquidity'
      }
    ];

    setExecutions(mockExecutions);
    
    // Calculate stats
    const completed = mockExecutions.filter(e => e.status === 'completed');
    const executing = mockExecutions.filter(e => e.status === 'executing');
    const failed = mockExecutions.filter(e => e.status === 'failed');
    const totalProfit = completed.reduce((sum, e) => sum + e.actualProfit, 0);
    const avgTime = completed.length > 0 
      ? completed.reduce((sum, e) => sum + e.executionTime, 0) / completed.length 
      : 0;

    setStats({
      total: mockExecutions.length,
      executing: executing.length,
      completed: completed.length,
      failed: failed.length,
      totalProfit,
      avgExecutionTime: avgTime
    });

    // Auto-refresh simulation
    if (isAutoRefresh) {
      const interval = setInterval(() => {
        // Simulate new executions or status updates
        setExecutions(prev => {
          const updated = [...prev];
          // Randomly update executing trades
          updated.forEach(exec => {
            if (exec.status === 'executing' && Math.random() > 0.7) {
              exec.status = Math.random() > 0.8 ? 'failed' : 'completed';
              exec.completedAt = new Date().toISOString();
              exec.executionTime = Date.now() - new Date(exec.startedAt).getTime();
              if (exec.status === 'completed') {
                exec.actualProfit = Math.random() * 50 + 10;
                exec.actualProfitPercent = (exec.actualProfit / exec.executedAmount) * 100;
                exec.txHash = '5KJp9X2vN8qR4mF7wE3tY6uH9sL2cV1bA8nM4xZ7pQ9rT3kW6jS';
              }
            }
          });
          return updated;
        });
      }, 5000);

      return () => clearInterval(interval);
    }
  }, [isAutoRefresh]);

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'executing': return <Clock className="h-4 w-4 text-yellow-400" />;
      case 'completed': return <CheckCircle className="h-4 w-4 text-primary-green" />;
      case 'failed': return <XCircle className="h-4 w-4 text-primary-red" />;
      default: return <Activity className="h-4 w-4 text-gray-400" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'executing': return 'text-yellow-400';
      case 'completed': return 'text-primary-green';
      case 'failed': return 'text-primary-red';
      default: return 'text-gray-400';
    }
  };

  const filteredExecutions = executions.filter(exec => 
    filter === 'all' || exec.status === filter
  );

  const formatDuration = (ms: number) => {
    if (ms < 1000) return `${ms}ms`;
    return `${(ms / 1000).toFixed(1)}s`;
  };

  const formatTxHash = (hash: string | null) => {
    if (!hash) return 'N/A';
    return `${hash.slice(0, 8)}...${hash.slice(-8)}`;
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-white">Execution Monitor</h1>
          <p className="text-gray-400 mt-1">Real-time monitoring of arbitrage executions</p>
        </div>
        <div className="flex items-center space-x-4">
          <button
            onClick={() => setIsAutoRefresh(!isAutoRefresh)}
            className={`flex items-center space-x-2 px-4 py-2 rounded-lg border transition-colors ${
              isAutoRefresh 
                ? 'bg-primary-green/10 border-primary-green text-primary-green' 
                : 'bg-dark-border border-gray-600 text-gray-400 hover:text-white'
            }`}
          >
            {isAutoRefresh ? <Pause className="h-4 w-4" /> : <Play className="h-4 w-4" />}
            <span>{isAutoRefresh ? 'Pause' : 'Resume'}</span>
          </button>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Total Executions</p>
              <p className="metric-value">{stats.total}</p>
            </div>
            <Activity className="h-8 w-8 text-primary-blue" />
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Success Rate</p>
              <p className="metric-value">
                {stats.total > 0 ? ((stats.completed / stats.total) * 100).toFixed(1) : 0}%
              </p>
            </div>
            <TrendingUp className="h-8 w-8 text-primary-green" />
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Total Profit</p>
              <p className="metric-value">${stats.totalProfit.toFixed(2)}</p>
            </div>
            <CheckCircle className="h-8 w-8 text-primary-green" />
          </div>
        </div>

        <div className="metric-card">
          <div className="flex items-center justify-between">
            <div>
              <p className="metric-label">Avg Execution</p>
              <p className="metric-value">{formatDuration(stats.avgExecutionTime)}</p>
            </div>
            <Clock className="h-8 w-8 text-yellow-400" />
          </div>
        </div>
      </div>

      {/* Status Filter */}
      <div className="flex items-center space-x-4">
        <Filter className="h-5 w-5 text-gray-400" />
        <div className="flex space-x-2">
          {[
            { key: 'all', label: 'All', count: stats.total },
            { key: 'executing', label: 'Executing', count: stats.executing },
            { key: 'completed', label: 'Completed', count: stats.completed },
            { key: 'failed', label: 'Failed', count: stats.failed }
          ].map(({ key, label, count }) => (
            <button
              key={key}
              onClick={() => setFilter(key as any)}
              className={`px-3 py-1 rounded-full text-sm font-medium transition-colors ${
                filter === key
                  ? 'bg-primary-green/10 text-primary-green border border-primary-green/20'
                  : 'bg-dark-border text-gray-400 hover:text-white'
              }`}
            >
              {label} ({count})
            </button>
          ))}
        </div>
      </div>

      {/* Executions Table */}
      <div className="card">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-dark-border">
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Status</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Pair</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Route</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Amount</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Profit</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Duration</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Transaction</th>
                <th className="text-left py-3 px-4 text-gray-400 font-medium">Started</th>
              </tr>
            </thead>
            <tbody>
              {filteredExecutions.map((execution) => (
                <tr key={execution.id} className="border-b border-dark-border hover:bg-dark-border/50">
                  <td className="py-3 px-4">
                    <div className="flex items-center space-x-2">
                      {getStatusIcon(execution.status)}
                      <span className={`text-sm font-medium capitalize ${getStatusColor(execution.status)}`}>
                        {execution.status}
                      </span>
                    </div>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-white font-medium">{execution.tokenPair}</span>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-gray-300 text-sm">
                      {execution.buyDex} → {execution.sellDex}
                    </span>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-white">${execution.executedAmount.toLocaleString()}</span>
                  </td>
                  <td className="py-3 px-4">
                    <div className="flex flex-col">
                      <span className={`font-medium ${
                        execution.actualProfit > 0 ? 'text-primary-green' : 'text-primary-red'
                      }`}>
                        ${execution.actualProfit.toFixed(2)}
                      </span>
                      {execution.actualProfitPercent !== 0 && (
                        <span className="text-xs text-gray-400">
                          {execution.actualProfitPercent.toFixed(2)}%
                        </span>
                      )}
                    </div>
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-gray-300">
                      {execution.status === 'executing' 
                        ? formatDuration(Date.now() - new Date(execution.startedAt).getTime())
                        : formatDuration(execution.executionTime)
                      }
                    </span>
                  </td>
                  <td className="py-3 px-4">
                    {execution.txHash ? (
                      <div className="flex items-center space-x-2">
                        <span className="text-primary-blue text-sm font-mono">
                          {formatTxHash(execution.txHash)}
                        </span>
                        <button className="text-gray-400 hover:text-primary-blue">
                          <ExternalLink className="h-3 w-3" />
                        </button>
                      </div>
                    ) : (
                      <span className="text-gray-500 text-sm">Pending</span>
                    )}
                  </td>
                  <td className="py-3 px-4">
                    <span className="text-gray-400 text-sm">
                      {new Date(execution.startedAt).toLocaleTimeString()}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        
        {filteredExecutions.length === 0 && (
          <div className="text-center py-8">
            <Activity className="h-12 w-12 text-gray-600 mx-auto mb-4" />
            <p className="text-gray-400">No executions found for the selected filter</p>
          </div>
        )}
      </div>

      {/* Error Log */}
      {executions.some(e => e.error) && (
        <div className="card">
          <div className="flex items-center space-x-2 mb-4">
            <AlertTriangle className="h-5 w-5 text-primary-red" />
            <h3 className="text-lg font-semibold text-white">Recent Errors</h3>
          </div>
          <div className="space-y-2">
            {executions
              .filter(e => e.error)
              .slice(0, 5)
              .map((execution) => (
                <div key={execution.id} className="p-3 bg-primary-red/10 border border-primary-red/20 rounded-lg">
                  <div className="flex items-center justify-between">
                    <div>
                      <span className="text-white font-medium">{execution.tokenPair}</span>
                      <span className="text-gray-400 ml-2 text-sm">
                        {execution.buyDex} → {execution.sellDex}
                      </span>
                    </div>
                    <span className="text-xs text-gray-400">
                      {new Date(execution.startedAt).toLocaleTimeString()}
                    </span>
                  </div>
                  <p className="text-primary-red text-sm mt-1">{execution.error}</p>
                </div>
              ))
            }
          </div>
        </div>
      )}
    </div>
  );
};

export default ExecutionMonitor;