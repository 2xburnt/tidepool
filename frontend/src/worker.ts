import { CONTRACTS, REST_ENDPOINT } from "./constants";

interface Env {
  ASSETS: Fetcher;
}

async function queryContract(contract: string, msg: object): Promise<unknown> {
  const encoded = btoa(JSON.stringify(msg));
  const resp = await fetch(
    `${REST_ENDPOINT}/cosmwasm/wasm/v1/contract/${contract}/smart/${encoded}`
  );
  if (!resp.ok) {
    throw new Error(`Chain query failed: ${resp.status} ${resp.statusText}`);
  }
  const json = (await resp.json()) as { data: unknown };
  return json.data;
}

function json(data: unknown, status = 200): Response {
  return new Response(JSON.stringify(data), {
    status,
    headers: {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      "Cache-Control": status === 200 ? "public, max-age=10" : "no-cache",
    },
  });
}

function error(message: string, status = 400): Response {
  return json({ error: message }, status);
}

async function handleApi(path: string, request: Request): Promise<Response> {
  try {
    switch (path) {
      case "/api/health":
        return json({ status: "ok", chain: "xion-testnet-2", contracts: CONTRACTS });

      case "/api/agents":
      case "/api/leaderboard": {
        const url = new URL(request.url);
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "50"), 100);
        const data = await queryContract(CONTRACTS.reputation, { leaderboard: { limit } });
        return json(data);
      }

      case "/api/tasks": {
        const url = new URL(request.url);
        const limit = Math.min(parseInt(url.searchParams.get("limit") || "50"), 100);
        const status_filter = url.searchParams.get("status") || undefined;
        const query: Record<string, unknown> = { limit };
        if (status_filter) query.status = status_filter;
        const data = await queryContract(CONTRACTS.tasks, { list_tasks: query });
        return json(data);
      }

      case "/api/agent": {
        const url = new URL(request.url);
        const address = url.searchParams.get("address");
        if (!address) return error("address parameter required");
        const data = await queryContract(CONTRACTS.reputation, { agent: { address } });
        return json(data);
      }

      case "/api/task": {
        const url = new URL(request.url);
        const id = url.searchParams.get("id");
        if (!id) return error("id parameter required");
        const data = await queryContract(CONTRACTS.tasks, { task: { task_id: parseInt(id) } });
        return json(data);
      }

      case "/api/badges": {
        const url = new URL(request.url);
        const address = url.searchParams.get("address");
        if (!address) return error("address parameter required");
        const agent = (await queryContract(CONTRACTS.reputation, { agent: { address } })) as {
          badges?: unknown[];
        };
        return json({ badges: agent?.badges || [] });
      }

      case "/api/stats": {
        const [agents, tasks] = await Promise.all([
          queryContract(CONTRACTS.reputation, { leaderboard: { limit: 100 } }) as Promise<{
            agents: { xp: number }[];
          }>,
          queryContract(CONTRACTS.tasks, { list_tasks: { limit: 100 } }) as Promise<{
            tasks: { status: string }[];
          }>,
        ]);
        const agentList = agents.agents || [];
        const taskList = tasks.tasks || [];
        return json({
          total_agents: agentList.length,
          total_tasks: taskList.length,
          total_xp: agentList.reduce((s: number, a: { xp: number }) => s + a.xp, 0),
          open_tasks: taskList.filter((t: { status: string }) => t.status === "open").length,
          completed_tasks: taskList.filter((t: { status: string }) => t.status === "completed")
            .length,
        });
      }

      default:
        return error("Unknown endpoint. Try /api/health, /api/agents, /api/tasks, /api/stats", 404);
    }
  } catch (e) {
    return error(e instanceof Error ? e.message : String(e), 500);
  }
}

export default {
  async fetch(request: Request, env: Env): Promise<Response> {
    const url = new URL(request.url);

    // CORS preflight
    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "GET, OPTIONS",
          "Access-Control-Allow-Headers": "Content-Type",
        },
      });
    }

    // API routes
    if (url.pathname.startsWith("/api/")) {
      return handleApi(url.pathname, request);
    }

    // Everything else → static assets (React dashboard)
    return env.ASSETS.fetch(request);
  },
};
