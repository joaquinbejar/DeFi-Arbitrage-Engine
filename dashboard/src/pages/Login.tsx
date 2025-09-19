import { useState } from 'react';
import { Wallet, Zap } from 'lucide-react';
import { toast } from 'sonner';
import { useAuthStore } from '../stores/authStore';

const Login = () => {
  const [isConnecting, setIsConnecting] = useState(false);
  const { login } = useAuthStore();

  const connectWallet = async () => {
    setIsConnecting(true);
    
    try {
      // Check if Phantom wallet is available
      if (!window.solana || !window.solana.isPhantom) {
        toast.error('Phantom wallet not found. Please install Phantom wallet.');
        return;
      }

      // Connect to wallet
      const response = await window.solana.connect();
      const publicKey = response.publicKey.toString();
      
      // Create message to sign
      const message = `Sign in to Arbitrage Engine\nTimestamp: ${Date.now()}`;
      const encodedMessage = new TextEncoder().encode(message);
      
      // Request signature
      const signedMessage = await window.solana.signMessage(encodedMessage, 'utf8');
      const signature = Array.from(signedMessage.signature).map(b => b.toString(16).padStart(2, '0')).join('');
      
      // Authenticate with backend
      await login(publicKey, signature, message);
      
      toast.success('Successfully connected to wallet!');
    } catch (error: any) {
      console.error('Wallet connection error:', error);
      if (error.code === 4001) {
        toast.error('Wallet connection rejected by user.');
      } else {
        toast.error('Failed to connect wallet. Please try again.');
      }
    } finally {
      setIsConnecting(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-dark-bg">
      <div className="max-w-md w-full space-y-8 p-8">
        <div className="text-center">
          <div className="flex justify-center">
            <div className="flex items-center space-x-2">
              <Zap className="h-12 w-12 text-primary-green" />
              <div className="text-left">
                <h1 className="text-3xl font-bold text-white">Arbitrage Engine</h1>
                <p className="text-sm text-gray-400">Solana DeFi Platform</p>
              </div>
            </div>
          </div>
          <h2 className="mt-6 text-2xl font-bold text-white">
            Connect Your Wallet
          </h2>
          <p className="mt-2 text-sm text-gray-400">
            Connect your Solana wallet to access the arbitrage dashboard
          </p>
        </div>
        
        <div className="mt-8 space-y-6">
          <button
            onClick={connectWallet}
            disabled={isConnecting}
            className="group relative w-full flex justify-center py-4 px-4 border border-transparent text-sm font-medium rounded-lg text-white bg-primary-green hover:bg-primary-green/90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-green disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <span className="absolute left-0 inset-y-0 flex items-center pl-3">
              <Wallet className="h-5 w-5 text-white" />
            </span>
            {isConnecting ? 'Connecting...' : 'Connect Phantom Wallet'}
          </button>
          
          <div className="text-center">
            <p className="text-xs text-gray-500">
              Don't have a wallet?{' '}
              <a 
                href="https://phantom.app/" 
                target="_blank" 
                rel="noopener noreferrer"
                className="text-primary-green hover:text-primary-green/80"
              >
                Download Phantom
              </a>
            </p>
          </div>
        </div>
        
        <div className="mt-8 border-t border-dark-border pt-6">
          <div className="text-center text-xs text-gray-500">
            <p>Supported roles:</p>
            <div className="mt-2 space-y-1">
              <p><span className="text-primary-green">●</span> System Operator - Full control</p>
              <p><span className="text-primary-blue">●</span> Risk Analyst - Monitoring & limits</p>
              <p><span className="text-gray-400">●</span> Observer - View only</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

// Extend Window interface for Phantom wallet
declare global {
  interface Window {
    solana?: {
      isPhantom?: boolean;
      connect(): Promise<{ publicKey: { toString(): string } }>;
      signMessage(message: Uint8Array, display?: string): Promise<{ signature: Uint8Array }>;
    };
  }
}

export default Login;