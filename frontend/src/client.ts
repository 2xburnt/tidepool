import { CHAIN_CONFIG } from "./config";

export interface Agent {
  address: string;
  name: string;
  xp: number;
  level: number;
  badges: { badge_type: string; issuer: string; issued_at: number; proof: string }[];
  tasks_completed: number;
  tasks_posted: number;
  registered_at: number;
}

export interface Task {
  id: number;
  title: string;
  description: string;
  poster: string;
  claimant: string | null;
  status: string;
  xp_reward: number;
  required_badges: string[];
  created_at: number;
  claimed_at: number | null;
  completed_at: number | null;
  expires_at: number | null;
  bounty: { denom: string; amount: string } | null;
}

async function queryContract<T>(contract: string, msg: object): Promise<T> {
  const encoded = btoa(JSON.stringify(msg));
  const resp = await fetch(
    `${CHAIN_CONFIG.restEndpoint}/cosmwasm/wasm/v1/contract/${contract}/smart/${encoded}`
  );
  if (!resp.ok) throw new Error(`Query failed: ${resp.status}`);
  const json = await resp.json();
  return json.data as T;
}

export async function fetchLeaderboard(limit = 20): Promise<Agent[]> {
  const data = await queryContract<{ agents: Agent[] }>(
    CHAIN_CONFIG.contracts.reputation,
    { leaderboard: { limit } }
  );
  return data.agents;
}

export async function fetchTasks(): Promise<Task[]> {
  const data = await queryContract<{ tasks: Task[] }>(
    CHAIN_CONFIG.contracts.tasks,
    { list_tasks: { limit: 50 } }
  );
  return data.tasks;
}
