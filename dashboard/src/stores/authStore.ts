import { create } from 'zustand';

interface User {
  id: string;
  publicKey: string;
  role: 'admin' | 'trader' | 'viewer';
  createdAt: string;
  lastLogin?: string;
  isActive: boolean;
}

interface AuthState {
  isAuthenticated: boolean;
  user: User | null;
  token: string | null;
  login: (publicKey: string, signature: string, message: string) => Promise<void>;
  logout: () => void;
  setUser: (user: User) => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  isAuthenticated: false,
  user: null,
  token: null,

  login: async (publicKey: string, signature: string, message: string) => {
    try {
      const response = await fetch('/api/auth/wallet', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          publicKey,
          signature,
          message,
        }),
      });

      if (!response.ok) {
        throw new Error('Authentication failed');
      }

      const data = await response.json();
      
      if (data.success) {
        set({
          isAuthenticated: true,
          token: data.token,
          user: {
            id: crypto.randomUUID(),
            publicKey,
            role: data.role,
            createdAt: new Date().toISOString(),
            isActive: true,
          },
        });
        
        // Store token in localStorage
        localStorage.setItem('auth_token', data.token);
      } else {
        throw new Error('Authentication failed');
      }
    } catch (error) {
      console.error('Login error:', error);
      throw error;
    }
  },

  logout: () => {
    localStorage.removeItem('auth_token');
    set({
      isAuthenticated: false,
      user: null,
      token: null,
    });
  },

  setUser: (user: User) => {
    set({ user });
  },
}));

// Initialize auth state from localStorage
const token = localStorage.getItem('auth_token');
if (token) {
  useAuthStore.setState({
    isAuthenticated: true,
    token,
  });
}