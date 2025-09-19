export interface User {
  id: string;
  publicKey: string;
  role: 'admin' | 'trader' | 'viewer';
  createdAt: string;
  lastLogin?: string;
  isActive: boolean;
}

export interface ArbitrageOpportunity {
  id: string;
  tokenPair: string;
  dexA: string;
  dexB: string;
  priceA: number;
  priceB: number;
  profitMargin: number;
  estimatedProfit: number;
  volume: number;
  timestamp: Date;
  status: 'active' | 'executed' | 'expired';
}

export interface TradeExecution {
  id: string;
  opportunityId: string;
  tokenPair: string;
  dex: string;
  type: 'buy' | 'sell';
  amount: number;
  price: number;
  fee: number;
  profit: number;
  status: 'pending' | 'completed' | 'failed';
  timestamp: Date;
  txHash?: string;
}

export interface ProfitMetrics {
  totalProfit: number;
  dailyProfit: number;
  weeklyProfit: number;
  monthlyProfit: number;
  profitMargin: number;
  successRate: number;
}

export interface StrategyConfig {
  minProfitMargin: number;
  maxSlippage: number;
  maxTradeSize: number;
  enabledDEXs: string[];
  enabledTokens: string[];
}

export interface SystemStatus {
  isRunning: boolean;
  uptime: number;
  lastUpdate: Date;
  connectedDEXs: number;
  activeOpportunities: number;
}

export interface WalletInfo {
  publicKey: string;
  balance: number;
  connected: boolean;
  provider: 'phantom' | 'solflare' | 'other';
}