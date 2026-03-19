import { CHAIN_CONFIG } from "./config";

export interface SkillRating {
  skill: string;
  rating: number; // 1-5 stars
  jobs_completed: number;
}

export interface Agent {
  address: string;
  name: string;
  metadata_uri?: string;
  category?: string;
  skills: SkillRating[];
  total_earned: string; // uxion amount
  total_spent: string; // uxion amount
  jobs_completed: number;
  jobs_posted: number;
  registered_at: number;
}

export interface Task {
  id: number;
  title: string;
  description: string;
  category?: string;
  required_skills: string[];
  poster: string;
  claimant: string | null;
  status: "open" | "claimed" | "submitted" | "completed" | "cancelled" | "expired";
  escrow_amount: string; // uxion amount
  created_at: number;
  claimed_at: number | null;
  submitted_at: number | null;
  completed_at: number | null;
  expires_at: number | null;
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

export async function fetchLeaderboard(limit = 50): Promise<Agent[]> {
  const data = await queryContract<{ agents: Agent[] }>(
    CHAIN_CONFIG.contracts.reputation,
    { get_leaderboard: { limit } }
  );
  return data.agents;
}

export async function fetchTasks(limit = 50): Promise<Task[]> {
  const data = await queryContract<{ tasks: Task[] }>(
    CHAIN_CONFIG.contracts.tasks,
    { get_tasks: { limit } }
  );
  return data.tasks.sort((a, b) => b.id - a.id);
}

// Helper to convert uxion to XION
export function uxionToXion(uxion: string | number): number {
  return Number(uxion) / 1_000_000;
}

// Helper to format XION amount
export function formatXion(uxion: string | number): string {
  const xion = uxionToXion(uxion);
  return xion.toFixed(xion < 10 ? 2 : 1);
}

// Helper to render star rating
export function renderStars(rating: number): string {
  const fullStars = Math.floor(rating);
  const emptyStars = 5 - fullStars;
  return "★".repeat(fullStars) + "☆".repeat(emptyStars);
}
