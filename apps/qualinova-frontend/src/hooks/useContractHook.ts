"use client";

import { Client, networks } from '../../packages/hello_world/src';
import { useState, useEffect } from 'react';

export function useContractHook(name: string) {
  const [result, setResult] = useState('');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    const run = async () => {
      try {
        setLoading(true);
        const contract = new Client({
          ...networks.testnet,
          rpcUrl: 'https://soroban-testnet.stellar.org:443',
        });

        const res = await contract.hello({ to: name });
        setResult(res.result.join(' '));
      } catch (err) {
        setError(err instanceof Error ? err : new Error('Unknown error occurred'));
      } finally {
        setLoading(false);
      }
    };

    run();
  }, [name]);

  return { result, loading, error };
}